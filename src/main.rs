use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use sysinfo::System;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;

#[derive(Clone, Serialize)]
struct CpuMetrics {
    collected_at: DateTime<Utc>,
    total_cpu_usage_percent: Option<f32>,
    uptime_seconds: u64,
}

#[derive(Clone)]
struct AppState {
    metrics: Arc<RwLock<CpuMetrics>>,
}

async fn get_metrics(State(state): State<AppState>) -> Json<CpuMetrics> {
    Json(state.metrics.read().await.clone())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut system = System::new();
    system.refresh_cpu_usage();
    let state = AppState {
        metrics: Arc::new(RwLock::new(CpuMetrics {
            collected_at: Utc::now(),
            total_cpu_usage_percent: None,
            uptime_seconds: System::uptime(),
        })),
    };

    let collector_state = state.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            system.refresh_cpu_usage();
            let sample = CpuMetrics {
                collected_at: Utc::now(),
                // Host-wide percentage normalized across all logical CPUs.
                total_cpu_usage_percent: Some((system.global_cpu_usage() * 100.0).round() / 100.0),
                uptime_seconds: System::uptime(),
            };
            *collector_state.metrics.write().await = sample;
        }
    });

    let app = Router::new()
        .route("/api/status", get(get_metrics))
        .route("/api/health", get(health))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let host = std::env::var("SERVER_STATUS_HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
    let port = std::env::var("SERVER_STATUS_PORT")
        .unwrap_or_else(|_| "3000".to_owned())
        .parse::<u16>()
        .expect("PORT must be a valid port number (0-65535)");
    let host: IpAddr = host.parse().expect("HOST must be a valid IP address");
    let address = SocketAddr::new(host, port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind HTTP listener");
    println!("CPU metrics API listening on http://{address}");
    axum::serve(listener, app)
        .await
        .expect("HTTP server failed");
}
