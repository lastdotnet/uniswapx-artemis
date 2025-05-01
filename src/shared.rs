use std::sync::Arc;

use alloy::{network::AnyNetwork, providers::{DynProvider, Provider}};
use alloy_primitives::Address;
use serde::Deserialize;

macro_rules! send_metric_with_order_hash {
    ($order_hash: expr, $future: expr) => {
        let hash = Arc::clone($order_hash);
        tokio::spawn(async move {
            if let Err(e) = $future.await {
                tracing::warn!("{} - error sending metric: {:?}", hash, e);
            }
        })
    };
}

macro_rules! u256 {
    ($($limb:expr),*) => {
        alloy_primitives::Uint::from_limbs([$($limb, 0, 0, 0),*])
    };
}

pub(crate) use send_metric_with_order_hash;
pub(crate) use u256;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub struct MethodParameters {
    pub calldata: String,
    pub value: String,
    pub to: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouteInfo {
    pub quote: String,
    #[serde(rename = "quoteGasAdjusted")]
    pub quote_gas_adjusted: String,
    #[serde(rename = "gasUseEstimate")]
    pub gas_use_estimate: String,
    #[serde(rename = "gasUseEstimateQuote")]
    pub gas_use_estimate_quote: String,
    #[serde(rename = "gasPriceWei")]
    pub gas_price_wei: String,
    #[serde(rename = "methodParameters")]
    pub method_parameters: MethodParameters,
}


pub async fn get_nonce_with_retry(
    sender_client: &Arc<DynProvider<AnyNetwork>>,
    address: Address,
    order_hash: &str,
    max_attempts: u32,
) -> Result<u64, anyhow::Error> {
    let mut attempts = 0;
    loop {
        match sender_client.get_transaction_count(address).await {
            Ok(nonce) => break Ok(nonce),
            Err(e) => {
                if attempts < max_attempts - 1 {
                    attempts += 1;
                } else {
                    return Err(anyhow::anyhow!(
                        "{} - Failed to get nonce after {} attempts: {}",
                        order_hash,
                        max_attempts,
                        e
                    ));
                }
            }
        }
    }
}