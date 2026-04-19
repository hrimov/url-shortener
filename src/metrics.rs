use axum::extract::State;
use axum::response::IntoResponse;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

pub fn install_recorder() -> PrometheusHandle {
    PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus recorder")
}

pub async fn render(State(handle): State<PrometheusHandle>) -> impl IntoResponse {
    handle.render()
}
