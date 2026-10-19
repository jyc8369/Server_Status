use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};
use sysinfo::System;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;

const SAMPLE_INTERVAL: Duration = Duration::from_secs(1);
const MAX_SAMPLE_AGE: Duration = Duration::from_secs(5);

#[derive(Clone, Serialize)]
struct CpuMetrics {
    collected_at: DateTime<Utc>,
    total_cpu_usage_percent: Option<f32>,
    uptime_seconds: u64,
}

struct MetricsSnapshot {
    metrics: CpuMetrics,
    // Monotonic time prevents wall-clock changes from affecting freshness checks.
    last_collected: Option<Instant>,
}

#[derive(Clone)]
struct AppState {
    metrics: Arc<RwLock<MetricsSnapshot>>,
}

async fn get_metrics(State(state): State<AppState>) -> Json<CpuMetrics> {
    Json(state.metrics.read().await.metrics.clone())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

fn sample_is_fresh(last_collected: Option<Instant>, now: Instant) -> bool {
    last_collected
        .is_some_and(|sample_time| now.saturating_duration_since(sample_time) <= MAX_SAMPLE_AGE)
}

async fn readiness(State(state): State<AppState>) -> impl IntoResponse {
    let ready = {
        let snapshot = state.metrics.read().await;
        sample_is_fresh(snapshot.last_collected, Instant::now())
    };

    if ready {
        (
            StatusCode::OK,
            Json(serde_json::json!({ "status": "ready" })),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "status": "not_ready" })),
        )
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut system = System::new();
    system.refresh_cpu_usage();
    let state = AppState {
        metrics: Arc::new(RwLock::new(MetricsSnapshot {
            metrics: CpuMetrics {
                collected_at: Utc::now(),
                total_cpu_usage_percent: None,
                uptime_seconds: System::uptime(),
            },
            last_collected: None,
        })),
    };

    let collector_state = state.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(SAMPLE_INTERVAL).await;
            system.refresh_cpu_usage();
            let sample = CpuMetrics {
                collected_at: Utc::now(),
                // Host-wide percentage normalized across all logical CPUs.
                total_cpu_usage_percent: Some((system.global_cpu_usage() * 100.0).round() / 100.0),
                uptime_seconds: System::uptime(),
            };
            *collector_state.metrics.write().await = MetricsSnapshot {
                metrics: sample,
                last_collected: Some(Instant::now()),
            };
        }
    });

    let app = Router::new()
        .route("/api/status", get(get_metrics))
        .route("/api/health", get(health))
        .route("/api/ready", get(readiness))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Bind locally unless the operator explicitly opts into network exposure.
    let host = std::env::var("SERVER_STATUS_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state(last_collected: Option<Instant>) -> AppState {
        AppState {
            metrics: Arc::new(RwLock::new(MetricsSnapshot {
                metrics: CpuMetrics {
                    collected_at: Utc::now(),
                    total_cpu_usage_percent: None,
                    uptime_seconds: 42,
                },
                last_collected,
            })),
        }
    }

    #[test]
    fn sample_freshness_requires_a_recent_sample() {
        let now = Instant::now();
        assert!(!sample_is_fresh(None, now));
        assert!(sample_is_fresh(Some(now), now));
        assert!(sample_is_fresh(Some(now - MAX_SAMPLE_AGE), now));
        assert!(!sample_is_fresh(
            Some(now - MAX_SAMPLE_AGE - Duration::from_millis(1)),
            now
        ));
    }

    #[tokio::test]
    async fn readiness_is_unavailable_before_first_sample() {
        let response = readiness(State(test_state(None))).await.into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn readiness_is_available_for_fresh_sample() {
        let response = readiness(State(test_state(Some(Instant::now()))))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn readiness_is_unavailable_for_stale_sample() {
        let last_collected = Instant::now() - MAX_SAMPLE_AGE - Duration::from_secs(1);
        let response = readiness(State(test_state(Some(last_collected))))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn status_keeps_its_existing_json_fields() {
        let Json(metrics) = get_metrics(State(test_state(None))).await;
        let json = serde_json::to_value(metrics).unwrap();
        assert!(json["collected_at"].is_string());
        assert!(json["total_cpu_usage_percent"].is_null());
        assert_eq!(json["uptime_seconds"], 42);
    }

    #[tokio::test]
    async fn health_reports_liveness_independently_of_collection() {
        assert_eq!(health().await.into_response().status(), StatusCode::OK);
    }
}
