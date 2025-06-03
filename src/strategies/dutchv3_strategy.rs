use super::{
    shared::UniswapXStrategy,
    types::{Config, OrderStatus, TokenInTokenOut},
};
use crate::{
    aws_utils::cloudwatch_utils::{
        CwMetrics, DimensionName, DimensionValue, MetricBuilder, PAWSWAP_NAMESPACE,
    },
    collectors::{
        block_collector::NewBlock,
        uniswapx_order_collector::UniswapXOrder,
        uniswapx_route_collector::{OrderBatchData, OrderData, RoutedOrder},
    },
    shared::RouteInfo,
};
use alloy::{
    hex,
    network::{AnyNetwork, TransactionBuilder},
    primitives::{Address, Bytes, Uint, U256},
    providers::{DynProvider, Provider},
    rpc::types::Filter,
};
use anyhow::Result;
use artemis_light::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_light::types::Strategy;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use bindings_uniswapx::base_reactor::BaseReactor::SignedOrder;

use std::str::FromStr;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};
use std::{
    error::Error,
    ops::{Div, Mul},
    sync::Arc,
};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info, warn};
use uniswapx_rs::order::{Order, OrderResolution, V3DutchOrder};

use super::types::{Action, Event};

const DONE_EXPIRY: u64 = 300;

#[derive(Debug)]
#[allow(dead_code)]
pub struct UniswapXDutchV3Fill {
    /// Ethers client.
    client: Arc<DynProvider<AnyNetwork>>,
    // AWS Cloudwatch CLient for metrics propagation
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
    /// executor address
    executor_address: String,
    /// Amount of profits to bid in gas
    bid_percentage: u64,
    last_block_number: u64,
    last_block_timestamp: u64,
    // map of open order hashes to order data
    open_orders: HashMap<String, OrderData>,
    // map of order hashes that are currently being processed (routed/executed)
    processing_orders: HashSet<String>,
    // map of done order hashes to time at which we can safely prune them
    done_orders: HashMap<String, u64>,
    batch_sender: Sender<Vec<OrderBatchData>>,
    route_receiver: Receiver<RoutedOrder>,
    sender_address: String,
    chain_id: u64,
    reactor_address: Address,
}

impl UniswapXDutchV3Fill {
    pub fn new(
        client: Arc<DynProvider<AnyNetwork>>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
        config: Config,
        sender: Sender<Vec<OrderBatchData>>,
        receiver: Receiver<RoutedOrder>,
        sender_address: String,
        chain_id: u64,
        reactor_address: Address,
    ) -> Self {
        info!("syncing state");

        Self {
            client,
            cloudwatch_client,
            executor_address: config.executor_address,
            bid_percentage: config
                .bid_percentage
                .expect("Config missing bid_percentage: cannot initialize UniswapXDutchV3Fill"),
            last_block_number: 0,
            last_block_timestamp: 0,
            open_orders: HashMap::new(),
            processing_orders: HashSet::new(),
            done_orders: HashMap::new(),
            batch_sender: sender,
            route_receiver: receiver,
            sender_address,
            chain_id,
            reactor_address,
        }
    }
}

#[async_trait]
impl Strategy<Event, Action> for UniswapXDutchV3Fill {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::UniswapXOrder(order) => self.process_order_event(&order).await,
            Event::NewBlock(block) => self.process_new_block_event(&block).await,
            Event::UniswapXRoute(route) => self.process_new_route(&route).await,
        }
    }
}

impl UniswapXStrategy for UniswapXDutchV3Fill {}

struct DutchV3OrderWrapper {
    inner: V3DutchOrder,
    encoded_order: String,
}

impl UniswapXDutchV3Fill {
    fn decode_order(&self, encoded_order: &str) -> Result<V3DutchOrder, Box<dyn Error>> {
        let encoded_order = if let Some(stripped) = encoded_order.strip_prefix("0x") {
            stripped
        } else {
            encoded_order
        };
        let order_hex: Vec<u8> = hex::decode(encoded_order)?;

        V3DutchOrder::decode_inner(&order_hex, false)
    }

    // Process new orders as they come in.
    async fn process_order_event(&mut self, event: &UniswapXOrder) -> Vec<Action> {
        if self.last_block_timestamp == 0 || self.processing_orders.contains(&event.order_hash) {
            return vec![];
        }

        let order = self
            .decode_order(&event.encoded_order)
            .map_err(|e| error!("failed to decode: {}", e))
            .ok();

        if let Some(order) = order {
            let wrapper = DutchV3OrderWrapper {
                inner: order,
                encoded_order: event.encoded_order.clone(),
            };
            self.update_order_state(
                wrapper,
                &event.signature,
                &event.order_hash,
                event.route.as_ref(),
            );
        }
        vec![]
    }

    async fn process_new_route(&mut self, event: &RoutedOrder) -> Vec<Action> {
        if event
            .request
            .orders
            .iter()
            .any(|o| self.done_orders.contains_key(&o.hash))
        {
            return vec![];
        }

        let OrderBatchData {
            orders,
            amount_required,
            ..
        } = &event.request;

        // Filter out orders that are already being processed
        let filtered_orders: Vec<OrderData> = orders
            .iter()
            .filter(|o| !self.processing_orders.contains(&o.hash))
            .cloned()
            .collect();
        if filtered_orders.is_empty() {
            return vec![];
        }

        let amount_required_u256 = U256::from_str_radix(&amount_required.to_string(), 10).ok();
        info!(
            "Quote: {:?}, Amount required: {:?}",
            event.route.quote_gas_adjusted, amount_required_u256
        );
        if let Some(profit) = self.get_profit_eth(event) {
            info!(
                "Sending trade: num trades: {} routed quote: {}, batch needs: {}, profit: {} wei",
                filtered_orders.len(),
                event.route.quote_gas_adjusted,
                amount_required.to_string(),
                profit
            );
            let signed_orders = self
                .get_signed_orders(filtered_orders.clone())
                .unwrap_or_else(|e| {
                    error!("Error getting signed orders: {}", e);
                    vec![]
                });
            let tx_request = self
                .build_fill(
                    self.client.clone(),
                    &self.executor_address,
                    signed_orders,
                    event,
                )
                .await;

            match tx_request {
                Ok(mut req) => {
                    // Must be able to cover min gas cost
                    let sender_address = Address::from_str(&self.sender_address).unwrap();
                    req.set_from(sender_address);
                    let gas_usage = self
                        .client
                        .estimate_gas(req.clone().into())
                        .await
                        .map_or_else(
                            |err| {
                                info!("Error estimating gas: {}", err);
                                if err.to_string().contains("execution reverted") {
                                    None
                                } else {
                                    Some(1_000_000)
                                }
                            },
                            Some,
                        );

                    if gas_usage.is_none() {
                        return vec![];
                    }
                    let gas_usage = gas_usage.unwrap();
                    // Get the current min gas price
                    let min_gas_price = self
                        .get_arbitrum_min_gas_price(self.client.clone())
                        .await
                        .unwrap_or(U256::from(10_000_000));

                    // gas price at which we'd break even, meaning 100% of profit goes to validator
                    let breakeven_gas_price = profit / U256::from(gas_usage);
                    // gas price corresponding to bid percentage
                    let bid_gas_price: Uint<256, 4> = breakeven_gas_price
                        .mul(U256::from(self.bid_percentage))
                        .div(U256::from(100));
                    if bid_gas_price < min_gas_price {
                        info!(
                            "Bid gas price {} is less than min gas price {}, skipping",
                            bid_gas_price, min_gas_price
                        );
                        return vec![];
                    }

                    for order in filtered_orders.iter() {
                        self.processing_orders.insert(order.hash.clone());
                    }
                    return vec![Action::SubmitTx(Box::new(SubmitTxToMempool {
                        tx: req,
                        gas_bid_info: Some(GasBidInfo {
                            bid_percentage: self.bid_percentage,
                            total_profit: profit.to(),
                        }),
                    }))];
                }
                Err(e) => {
                    error!("Error building fill: {}", e);
                    return vec![];
                }
            }
        }
        vec![]
    }

    /// Process new block events, updating the internal state.
    async fn process_new_block_event(&mut self, event: &NewBlock) -> Vec<Action> {
        self.last_block_number = event.number;
        self.last_block_timestamp = event.timestamp;

        info!(
            "Processing block {} at {}, Order set sizes -- open: {}, processing: {}, done: {}",
            event.number,
            event.timestamp,
            self.open_orders.len(),
            self.processing_orders.len(),
            self.done_orders.len()
        );
        self.handle_fills().await.unwrap_or_else(|e| {
            error!("Error handling fills: {}", e);
        });
        self.update_open_orders();
        self.prune_done_orders();

        self.batch_sender
            .send(self.get_order_batches().values().cloned().collect())
            .await
            .unwrap_or_else(|e| {
                error!("Error sending order batches: {}", e);
            });

        vec![]
    }

    /// encode orders into generic signed orders
    fn get_signed_orders(&self, orders: Vec<OrderData>) -> Result<Vec<SignedOrder>> {
        let mut signed_orders: Vec<SignedOrder> = Vec::new();
        for batch in orders.iter() {
            signed_orders.push(SignedOrder {
                order: Bytes::from_str(batch.encoded_order.as_ref().unwrap())?,
                sig: Bytes::from_str(&batch.signature)?,
            });
        }
        Ok(signed_orders)
    }

    fn get_order_batches(&self) -> HashMap<TokenInTokenOut, OrderBatchData> {
        let mut order_batches: HashMap<TokenInTokenOut, OrderBatchData> = HashMap::new();

        // group orders by token in, token out, and order type (exact_in or exact_out)
        self.open_orders
            .iter()
            .filter(|(_, order_data)| !self.processing_orders.contains(&order_data.hash))
            .for_each(|(_, order_data)| {
                let token_in_token_out = TokenInTokenOut {
                    token_in: order_data.resolved.input.token.clone(),
                    token_out: order_data.resolved.outputs[0].token.clone(),
                };

                let amount_in = order_data.resolved.input.amount;
                let amount_out = order_data
                    .resolved
                    .outputs
                    .iter()
                    .fold(Uint::from(0), |sum, output| sum.wrapping_add(output.amount));

                let amount_required = if order_data.order.is_exact_output() {
                    amount_in
                } else {
                    amount_out
                };
                // insert new order and update total amount out
                if let std::collections::hash_map::Entry::Vacant(e) =
                    order_batches.entry(token_in_token_out.clone())
                {
                    e.insert(OrderBatchData {
                        orders: vec![order_data.clone()],
                        amount_in,
                        amount_out,
                        amount_required,
                        token_in: order_data.resolved.input.token.clone(),
                        token_out: order_data.resolved.outputs[0].token.clone(),
                        chain_id: self.chain_id,
                    });
                } else {
                    let order_batch_data = order_batches.get_mut(&token_in_token_out).unwrap();
                    order_batch_data.orders.push(order_data.clone());
                    order_batch_data.amount_in = order_batch_data.amount_in.wrapping_add(amount_in);
                    order_batch_data.amount_required = order_batch_data
                        .amount_required
                        .wrapping_add(amount_required);
                }
            });

        order_batches
    }

    async fn handle_fills(&mut self) -> Result<()> {
        let filter = Filter::new()
            .select(self.last_block_number)
            .address(self.reactor_address)
            .event("Fill(bytes32,address,address,uint256)");

        // early return on error
        let logs = self.client.get_logs(&filter).await?;
        for log in logs {
            let order_hash = format!("0x{:x}", log.topics()[1]);
            // remove from open
            info!("{} - Removing filled order", order_hash);
            self.open_orders.remove(&order_hash);
            // add to done
            self.done_orders.insert(
                order_hash.to_string(),
                self.current_timestamp()? + DONE_EXPIRY,
            );
        }

        Ok(())
    }

    fn prune_done_orders(&mut self) {
        let mut to_remove = Vec::new();
        for (order_hash, deadline) in self.done_orders.iter() {
            if *deadline < self.last_block_timestamp {
                to_remove.push(order_hash.clone());
            }
        }
        for order_hash in to_remove {
            self.done_orders.remove(&order_hash);
        }
    }

    fn update_open_orders(&mut self) {
        // TODO: this is nasty, plz cleanup
        let binding = self.open_orders.clone();
        let order_hashes: Vec<(&String, &OrderData)> = binding.iter().collect();
        for (order_hash, order_data) in order_hashes {
            match &order_data.order {
                Order::V3DutchOrder(order) => {
                    self.update_order_state(
                        DutchV3OrderWrapper {
                            inner: order.clone(),
                            encoded_order: order_data.encoded_order.clone().unwrap(),
                        },
                        &order_data.signature,
                        &order_hash.to_string(),
                        order_data.route.as_ref(),
                    );
                }
                _ => {
                    error!("Invalid order type");
                }
            }
        }
    }

    fn mark_as_done(&mut self, order: &str) {
        if self.open_orders.contains_key(order) {
            self.open_orders.remove(order);
            self.processing_orders.remove(order);
        }
        if !self.done_orders.contains_key(order) {
            self.done_orders
                .insert(order.to_string(), self.last_block_timestamp + DONE_EXPIRY);
        }
    }

    fn update_order_state(
        &mut self,
        order: DutchV3OrderWrapper,
        signature: &str,
        order_hash: &String,
        route: Option<&RouteInfo>,
    ) {
        let resolved = order
            .inner
            .resolve(self.last_block_number, self.last_block_timestamp);
        let order_status: OrderStatus = match resolved {
            OrderResolution::Expired => OrderStatus::Done,
            OrderResolution::Invalid => OrderStatus::Done,
            OrderResolution::Resolved(resolved_order) => OrderStatus::Open(resolved_order),
            _ => OrderStatus::Done,
        };

        match order_status {
            OrderStatus::Done => {
                self.mark_as_done(order_hash);
            }
            OrderStatus::Open(resolved_order) => {
                if self.done_orders.contains_key(order_hash) {
                    info!("{} - Order already done, skipping", order_hash);
                    return;
                }
                if !self.open_orders.contains_key(order_hash) {
                    info!("{} - Adding new order", order_hash);

                    if let Some(cw) = &self.cloudwatch_client {
                        let metric_future = cw
                            .put_metric_data()
                            .namespace(PAWSWAP_NAMESPACE)
                            .metric_data(
                                MetricBuilder::new(CwMetrics::OrderReceived(self.chain_id))
                                    .add_dimension(
                                        DimensionName::Service.as_ref(),
                                        DimensionValue::V3Executor.as_ref(),
                                    )
                                    .with_value(1.0)
                                    .build(),
                            )
                            .send();
                        tokio::spawn(async move {
                            if let Err(e) = metric_future.await {
                                warn!("Error sending order received metric: {:?}", e);
                            }
                        });
                    }

                    self.open_orders.insert(
                        order_hash.clone(),
                        OrderData {
                            order: Order::V3DutchOrder(order.inner),
                            hash: order_hash.clone(),
                            signature: signature.to_string(),
                            resolved: resolved_order,
                            encoded_order: Some(order.encoded_order),
                            route: route.cloned(),
                        },
                    );
                }
            }
            // Noop
            _ => {}
        }
    }
}
