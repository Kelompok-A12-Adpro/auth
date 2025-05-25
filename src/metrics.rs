use actix_web_prom::{PrometheusMetricsBuilder, PrometheusMetrics};

pub fn setup_metrics() -> PrometheusMetrics {
    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .expect("failed to create PrometheusMetrics")
}
