use crate::executors::public_1559_executor::Public1559Executor;
use crate::strategies::keystore::KeyStore;
use crate::strategies::types::SubmitTxToMempoolWithExecutionMetadata;
use alloy::network::AnyNetwork;
use alloy::providers::Provider;
use alloy::transports::Transport;
use artemis_core::types::Executor;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct QueuedExecutor<P, T>
where
    P: Provider<T, AnyNetwork> + 'static,
    T: Transport + Clone + 'static,
{
    provider: Arc<P>,
    sender_client: Arc<P>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
    _transport: PhantomData<T>,
}

impl<P, T> QueuedExecutor<P, T>
where
    P: Provider<T, AnyNetwork> + 'static,
    T: Transport + Clone + 'static,
{
    pub fn new(
        provider: Arc<P>,
        sender_client: Arc<P>,
        key_store: Arc<KeyStore>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
    ) -> Self {
        Self {
            provider,
            sender_client,
            key_store,
            cloudwatch_client,
            _transport: PhantomData,
        }
    }
}

#[async_trait]
impl<P, T> Executor<SubmitTxToMempoolWithExecutionMetadata> for QueuedExecutor<P, T>
where
    P: Provider<T, AnyNetwork> + 'static,
    T: Transport + Clone + 'static,
{
    async fn execute(
        &self,
        action: SubmitTxToMempoolWithExecutionMetadata,
    ) -> Result<(), anyhow::Error> {
        let public_executor = Public1559Executor::<P, T>::new(
            self.provider.clone(),
            self.sender_client.clone(),
            self.key_store.clone(),
            self.cloudwatch_client.clone(),
        );

        tokio::spawn(async move { public_executor.execute(action).await });

        Ok(())
    }
}
