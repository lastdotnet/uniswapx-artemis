pub mod cloudwatch_utils {
    pub enum CwMetrics {
        TxSucceeded,
        TxReverted,
        TxSubmitted,
        TxStatusUnknown,
    }
    
    impl Into<String> for CwMetrics {
        fn into(self) -> String {
            match self {
                CwMetrics::TxSucceeded => "TransactionSucceeded".to_string(),
                CwMetrics::TxReverted => "TransactionReverted".to_string(),
                CwMetrics::TxSubmitted => "TransactionSubmitted".to_string(),
                CwMetrics::TxStatusUnknown => "TransactionStatusUnknown".to_string(),
            }
        }
    } 
    
    use aws_sdk_cloudwatch::types::Dimension;
    
    pub const NAMESPACE: &str = "Artemis";

    pub fn executor_dimension(value: &str) -> Dimension {
        Dimension::builder()
            .name("Executor")
            .value(value)
            .build()
    }
    
    pub fn receipt_status_to_metric(status: u64) -> CwMetrics {
        match status {
            1 => {
                CwMetrics::TxSucceeded
            }
            0 => {
                CwMetrics::TxReverted
            }
            _ => {
                CwMetrics::TxStatusUnknown
            }
        }
    }
}
