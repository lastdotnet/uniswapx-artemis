use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::JsonRpcClient,
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
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
/// Event stream will try to reconnect to the WSS connection with rpc node in case it's closed for unexpected reasons.
#[async_trait]
impl<M> Collector<NewBlock> for BlockCollector<M>
where
    M: Middleware,
    M::Provider: JsonRpcClient,
    M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        let mut watcher = self
            .provider
            .watch_blocks()
            .await
            .unwrap()
            .interval(Duration::from_millis(500))
            .stream();
        let stream = async_stream::stream! {
            loop {
                if let Some(block_hash) = watcher.next().await {
                    match self.provider.get_block(block_hash).await {
                        Ok(Some(block)) => {
                            yield NewBlock {
                                hash: block.hash.unwrap(),
                                number: block.number.unwrap(),
                                timestamp: block.timestamp,
                            };
                        }
                        _ => continue,
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
