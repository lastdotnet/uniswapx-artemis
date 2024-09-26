use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::PubsubClient,
    types::{H256, U256, U64},
};
use std::{sync::Arc, time::Duration};
use tokio_stream::StreamExt;

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector<M> {
    provider: Arc<M>,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub hash: H256,
    pub number: U64,
    pub timestamp: U256,
}

impl<M> BlockCollector<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }

    async fn exponential_backoff(attempt: u32) {
        let delay = std::cmp::min(2u64.pow(attempt) * 100, 2_000); // Max delay of 2 seconds
        tokio::time::sleep(Duration::from_millis(delay)).await;
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
/// Event stream will try to reconnect to the WSS connection with rpc node in case it's closed for unexpected reasons.
#[async_trait]
impl<M> Collector<NewBlock> for BlockCollector<M>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        let stream = async_stream::stream! {
            let mut attempt = 0;
            loop {
                match self.provider.subscribe_blocks().await {
                    Ok(mut stream) => {
                        tracing::info!("Successfully subscribed to new blocks stream");
                        while let Some(block)= stream.next().await {
                            if let (Some(hash), Some(number)) = (block.hash, block.number) {
                                yield NewBlock {
                                    hash,
                                    number,
                                    timestamp: block.timestamp,
                                };
                            } else {
                                tracing::warn!("Received block with missing hash or number: {:?}", block);
                            }
                        }
                        tracing::error!("New block stream ended unexpectedly");
                    }
                    Err(e) => {
                        tracing::error!("Failed to subscribe to new blocks: {:?}", e);
                        Self::exponential_backoff(attempt).await;
                        attempt += 1;
                    }
                }
            }
        };
        Ok(Box::pin(stream))
    }
}
