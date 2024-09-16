pub mod cloudwatch_utils {
    pub enum DimensionName {
        Executor,
    }
    impl AsRef<str> for DimensionName {
        fn as_ref(&self) -> &str {
            match self {
                DimensionName::Executor => "Executor",
            }
        }
    }

    impl Into<String> for DimensionName {
        fn into(self) -> String {
            match self {
                DimensionName::Executor => "Executor".to_string(),
            }
        }
    }

    pub enum DimensionValue {
        PriorityExecutor,
        V2Executor,
    }

    impl Into<String> for DimensionValue {
        fn into(self) -> String {
            match self {
                DimensionValue::PriorityExecutor => "PriorityExecutor".to_string(),
                DimensionValue::V2Executor => "V2Executor".to_string(),
            }
        }
    }

    impl AsRef<str> for DimensionValue {
        fn as_ref(&self) -> &str {
            match self {
                DimensionValue::PriorityExecutor => "PriorityExecutor",
                DimensionValue::V2Executor => "V2Executor",
            }
        }
    }

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

    pub const ARTEMIS_NAMESPACE: &str = "Artemis";

    pub struct MetricBuilder {
        metric_name: String,
        dimensions: Vec<Dimension>,
        value: f64,
    }

    impl MetricBuilder {
        pub fn new(metric: CwMetrics) -> Self {
            Self {
                metric_name: metric.into(),
                dimensions: Vec::new(),
                value: 1.0,
            }
        }

        pub fn add_dimension(mut self, name: &str, value: &str) -> Self {
            self.dimensions
                .push(Dimension::builder().name(name).value(value).build());
            self
        }

        pub fn with_value(mut self, value: f64) -> Self {
            self.value = value;
            self
        }

        pub fn build(self) -> aws_sdk_cloudwatch::types::MetricDatum {
            aws_sdk_cloudwatch::types::MetricDatum::builder()
                .metric_name(self.metric_name)
                .value(self.value)
                .set_dimensions(Some(self.dimensions))
                .build()
        }
    }

    pub fn receipt_status_to_metric(status: u64) -> CwMetrics {
        match status {
            1 => CwMetrics::TxSucceeded,
            0 => CwMetrics::TxReverted,
            _ => CwMetrics::TxStatusUnknown,
        }
    }
}
