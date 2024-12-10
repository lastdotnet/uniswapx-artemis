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

pub(crate) use send_metric_with_order_hash;
