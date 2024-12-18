use std::{
    ops::{Div, Mul},
    sync::Arc,
};
use tracing::{info, warn};

use anyhow::Result;
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::{
    middleware::MiddlewareBuilder,
    providers::Middleware,
    signers::{LocalWallet, Signer},
    types::{TransactionReceipt, U256}, utils::format_units,
};
use aws_sdk_cloudwatch::Client as CloudWatchClient;

use crate::{
    aws_utils::cloudwatch_utils::{
        receipt_status_to_metric, CwMetrics, DimensionName, DimensionValue, MetricBuilder,
        ARTEMIS_NAMESPACE,
    },
    strategies::keystore::KeyStore,
    send_metric,
};

/// An executor that sends transactions to the mempool.
pub struct ProtectExecutor<M, N> {
    client: Arc<M>,
    sender_client: Arc<N>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
}

impl<M: Middleware, N: Middleware> ProtectExecutor<M, N> {
    pub fn new(
        client: Arc<M>,
        sender_client: Arc<N>,
        key_store: Arc<KeyStore>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
    ) -> Self {
        Self {
            client,
            sender_client,
            key_store,
            cloudwatch_client,
        }
    }
}

#[async_trait]
impl<M, N> Executor<SubmitTxToMempool> for ProtectExecutor<M, N>
where
    M: Middleware,
    M::Error: 'static,
    N: Middleware + 'static,
    N::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool) -> Result<()> {
        // Acquire a key from the key store
        let (public_address, private_key) = self
            .key_store
            .acquire_key()
            .await
            .expect("Failed to acquire key");
        info!("Acquired key: {}", public_address);

        let chain_id = u64::from_str_radix(
            &action
                .tx
                .chain_id()
                .expect("Chain ID not found on transaction")
                .to_string(),
            10,
        )
        .expect("Failed to parse chain ID");

        let wallet: LocalWallet = private_key
            .as_str()
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id);
        let address = wallet.address();
        action.tx.set_from(address);
        let gas_usage_result = self
            .client
            .estimate_gas(&action.tx, None)
            .await
            .unwrap_or_else(|err| {
                info!("Error estimating gas: {}", err);
                U256::from(1_000_000)
            });
        info!("Gas Usage {:?}", gas_usage_result);
        let gas_usage = gas_usage_result;

        let bid_gas_price;
        if let Some(gas_bid_info) = action.gas_bid_info {
            // gas price at which we'd break even, meaning 100% of profit goes to validator
            let breakeven_gas_price = gas_bid_info.total_profit / gas_usage;
            // gas price corresponding to bid percentage
            bid_gas_price = breakeven_gas_price
                .mul(gas_bid_info.bid_percentage)
                .div(100);
        } else {
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .map_err(|err| anyhow::anyhow!("Error getting gas price: {}", err))?;
        }
        info!("bid_gas_price: {}", bid_gas_price);
        action.tx.set_gas_price(bid_gas_price);

        let sender_client = self.sender_client.clone();
        let nonce_manager = sender_client.nonce_manager(address);
        let signer = nonce_manager.with_signer(wallet);

        info!("Executing tx {:?}", action.tx);
        if let Some(cw) = &self.cloudwatch_client {
            let metric_future = cw
                .put_metric_data()
                .namespace(ARTEMIS_NAMESPACE)
                .metric_data(
                    MetricBuilder::new(CwMetrics::TxSubmitted)
                        .add_dimension(
                            DimensionName::Service.as_ref(),
                            DimensionValue::V3Executor.as_ref(),
                        )
                        .with_value(1.0)
                        .build(),
                )
                .send();

            // do not block current thread by awaiting in the background
            send_metric!(metric_future);
        }
        let result = signer.send_transaction(action.tx, None).await;

        // Block on pending transaction getting confirmations
        let (receipt, status) = match result {
            Ok(tx) => {
                let receipt = tx.confirmations(1).await.map_err(|e| {
                    anyhow::anyhow!("Error waiting for confirmations: {}", e)
                });
                match receipt {
                    Ok(Some(receipt)) => {
                        let status = receipt.status.unwrap_or_default();
                        info!(
                            "receipt: tx_hash: {:?}, status: {}",
                            receipt.transaction_hash, status,
                        );
                        (Some(receipt), status)
                    }
                    Ok(None) => {
                        warn!("No receipt after confirmation");
                        (None, ethers::types::U64::zero())
                    }
                    Err(e) => {
                        warn!("Error waiting for confirmations: {}", e);
                        (None, ethers::types::U64::zero())
                    }
                }
            }
            Err(e) => {
                warn!("Error sending transaction: {}", e);
                (None, ethers::types::U64::zero())
            }
        };

        match self.key_store.release_key(public_address.clone()).await {
            Ok(_) => {
                info!("Released key: {}", public_address);
            }
            Err(e) => {
                info!("Failed to release key: {}", e);
            }
        }

        // post key-release processing
        // TODO: parse revert reason
        if let Some(cw) = &self.cloudwatch_client {
            let metric_future = cw
                .put_metric_data()
                .namespace(ARTEMIS_NAMESPACE)
                .metric_data(
                    MetricBuilder::new(receipt_status_to_metric(status.as_u64()))
                        .add_dimension(
                            DimensionName::Service.as_ref(),
                            DimensionValue::V3Executor.as_ref(),
                        )
                        .with_value(1.0)
                        .build(),
                )
                .send();

            send_metric!(metric_future);
        }

        if let Some(TransactionReceipt {
            block_number: Some(block_number),
            ..
        }) = receipt
        {
            let balance_eth = self
                .client
                .get_balance(address, Some(block_number.into()))
                .await
                .map_or_else(|_| None, |v| Some(format_units(v, "ether").unwrap()));

            // TODO: use if-let chains when it becomes stable https://github.com/rust-lang/rust/issues/53667
            // if let Some(balance_eth) = balance_eth && let Some(cw) = &self.cloudwatch_client {
            if let Some(balance_eth) = balance_eth {
                info!(
                    "balance: {} at block {}",
                    balance_eth.clone(),
                    block_number.as_u64()
                );
                if let Some(cw) = &self.cloudwatch_client {
                    let metric_future = cw
                        .put_metric_data()
                        .namespace(ARTEMIS_NAMESPACE)
                        .metric_data(
                            MetricBuilder::new(CwMetrics::Balance(format!("{:?}", address))) // {:?} gives the full 0x-prefixed address
                                .add_dimension(
                                    DimensionName::Service.as_ref(),
                                    DimensionValue::V3Executor.as_ref(),
                                )
                                .with_value(balance_eth.parse::<f64>().unwrap_or(0.0))
                                .build(),
                        )
                        .send();
                    send_metric!(metric_future);
                }
            }
        }

        Ok(())
    }
}
