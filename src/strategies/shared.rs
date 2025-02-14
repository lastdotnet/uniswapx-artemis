use crate::collectors::uniswapx_route_collector::RoutedOrder;
use alloy::{
    consensus::TypedTransaction,
    dyn_abi::{abi::Token, DynSolType, DynSolValue},
    network::AnyNetwork,
    primitives::{Address, U256},
    providers::{DynProvider, Provider},
    sol,
    transports::Transport,
};
use alloy_primitives::Bytes;
use anyhow::Result;
use async_trait::async_trait;
use bindings_uniswapx::{
    basereactor::BaseReactor::SignedOrder, erc20::ERC20, swaprouter02executor::SwapRouter02Executor,
};
//use ethers::{
//    abi::{ethabi, ParamType, Token},
//    providers::Middleware,
//    types::{
//        transaction::eip2718::TypedTransaction, Address, Bytes, Eip1559TransactionRequest, H160,
//        U256,
//    },
//};
use std::sync::Arc;
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

const REACTOR_ADDRESS: &str = "0x00000011F84B9aa48e5f8aA8B9897600006289Be";
const SWAPROUTER_02_ADDRESS: &str = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45";
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const ARBITRUM_GAS_PRECOMPILE: &str = "0x000000000000000000000000000000000000006C";

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract GasPrecompileContract {
        function getMinimumGasPrice() external view returns (uint256);
    }
}

#[async_trait]
pub trait UniswapXStrategy
{
    // builds a transaction to fill an order
    async fn build_fill(
        &self,
        client: Arc<DynProvider<AnyNetwork>>,
        executor_address: &str,
        signed_orders: Vec<SignedOrder>,
        RoutedOrder { request, route, .. }: &RoutedOrder,
    ) -> Result<TypedTransaction> {
        let multicall_type: DynSolType = DynSolType::Tuple(vec![
            DynSolType::Uint(256),
            DynSolType::Array(Box::new(DynSolType::Bytes)),
        ]);

        let calldata_type: DynSolType = DynSolType::Tuple(vec![
            DynSolType::Array(Box::new(DynSolType::Address)),
            DynSolType::Array(Box::new(DynSolType::Address)),
            DynSolType::Array(Box::new(DynSolType::Bytes)),
        ]);
        let chain_id = client.get_chain_id().await?;
        let fill_contract =
            SwapRouter02Executor::new(Address::from_str(executor_address)?, client.clone());

        let token_in = Address::from_str(&request.token_in)?;
        let token_out = Address::from_str(&request.token_out)?;

        let swaprouter_02_approval = self
            .get_tokens_to_approve(
                client.clone(),
                token_in,
                executor_address,
                SWAPROUTER_02_ADDRESS,
            )
            .await?;

        let reactor_approval = self
            .get_tokens_to_approve(client.clone(), token_out, executor_address, REACTOR_ADDRESS)
            .await?;

        // Strip off function selector
        let multicall_bytes = &route.method_parameters.calldata[10..];

        // Decode multicall into [Uint256, bytes[]] (deadline, multicallData)
        let decoded_multicall_bytes = multicall_type.abi_decode(
            &Bytes::from_str(multicall_bytes).expect("Failed to decode multicall bytes"),
        );

        let decoded_multicall_bytes = match decoded_multicall_bytes {
            Ok(data) => {
                if let DynSolValue::Tuple(values) = data {
                    values[1].clone()
                } else {
                    return Err(anyhow::anyhow!("Failed to decode multicall bytes"));
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to decode multicall bytes: {}", e));
            }
        };

        // abi encode as [tokens to approve to swap router 02, tokens to approve to reactor,  multicall data]
        //               [address[], address[], bytes[]]
        let calldata = DynSolValue::Tuple(vec![
            DynSolValue::Array(swaprouter_02_approval.iter().map(|a| DynSolValue::Address(a.clone())).collect()),
            DynSolValue::Array(reactor_approval.iter().map(|a| DynSolValue::Address(a.clone())).collect()),
            DynSolValue::Array(vec![decoded_multicall_bytes]),
        ]);
        let encoded_calldata = calldata.abi_encode();
        let orders: Vec<SwapRouter02Executor::SignedOrder> =
            signed_orders.into_iter().map(|order| {
                SwapRouter02Executor::SignedOrder {
                    order: order.order,
                    sig: order.sig,
                }
            }).collect();
        let mut call = fill_contract.executeBatch(orders, Bytes::from(encoded_calldata));
        Ok(call.tx.set_chain_id(chain_id).clone())
    }

    fn current_timestamp(&self) -> Result<u64> {
        let start = SystemTime::now();
        Ok(start.duration_since(UNIX_EPOCH)?.as_secs())
    }

    async fn get_tokens_to_approve(
        &self,
        client: Arc<P>,
        token: Address,
        from: &str,
        to: &str,
    ) -> Result<Vec<Address>, anyhow::Error> {
        if token == Address::zero() {
            return Ok(vec![]);
        }
        let token_contract = ERC20::new(token, client.clone());
        let allowance = token_contract
            .allowance(
                Address::from_str(from).expect("Error encoding from address"),
                Address::from_str(to).expect("Error encoding from address"),
            )
            .await
            .expect("Failed to get allowance");
        if allowance < U256::MAX / 2 {
            Ok(vec![token])
        } else {
            Ok(vec![])
        }
    }

    fn get_profit_eth(&self, RoutedOrder { request, route, .. }: &RoutedOrder) -> Option<U256> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;
        let amount_out_required =
            U256::from_str_radix(&request.amount_out_required.to_string(), 10).ok()?;
        if quote.le(&amount_out_required) {
            return None;
        }
        let profit_quote = quote.saturating_sub(amount_out_required);

        if request.token_out.to_lowercase() == WETH_ADDRESS.to_lowercase() {
            return Some(profit_quote);
        }

        let gas_use_eth = U256::from_str_radix(&route.gas_use_estimate, 10)
            .ok()?
            .saturating_mul(U256::from_str_radix(&route.gas_price_wei, 10).ok()?);
        profit_quote
            .saturating_mul(gas_use_eth)
            .checked_div(U256::from_str_radix(&route.gas_use_estimate_quote, 10).ok()?)
    }

    /// Get the minimum gas price on Arbitrum
    /// https://docs.arbitrum.io/build-decentralized-apps/precompiles/reference#arbgasinfo
    async fn get_arbitrum_min_gas_price(&self, client: Arc<P>) -> Result<U256> {
        let precompile_address = ARBITRUM_GAS_PRECOMPILE.parse::<Address>()?;
        let gas_precompile = GasPrecompileContract::new(precompile_address, client.clone());
        let gas_info = gas_precompile.getMinimumGasPrice().call().await?._0;

        Ok(gas_info)
    }
}
