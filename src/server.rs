use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::generator::{GeneratorConfig, GeneratorState, start_generation};

pub type AppState = Arc<GeneratorState>;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_page))
        .route("/api/start", post(start_handler))
        .route("/api/stop", post(stop_handler))
        .route("/api/progress", get(progress_handler))
        .with_state(state)
}

async fn index_page() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

#[derive(Deserialize)]
struct StartRequest {
    num_services: u32,
    total_size_mb: u64,
    file_max_size_mb: u64,
}

#[derive(Serialize)]
struct StartResponse {
    message: String,
}

#[derive(Serialize)]
struct ProgressResponse {
    running: bool,
    bytes_written: u64,
    target_bytes: u64,
    percent: f64,
    services_total: u64,
    services_done: u64,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn start_handler(
    State(state): State<AppState>,
    Json(req): Json<StartRequest>,
) -> Result<Json<StartResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate
    if req.num_services == 0 || req.num_services > 1000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Number of services must be between 1 and 1000".into(),
            }),
        ));
    }
    if req.total_size_mb == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Total size must be greater than 0".into(),
            }),
        ));
    }
    if req.file_max_size_mb == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "File max size must be greater than 0".into(),
            }),
        ));
    }

    // Check if already running
    if state.running.load(Ordering::SeqCst) {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Generation is already running. Stop it first.".into(),
            }),
        ));
    }

    let target_bytes = req.total_size_mb * 1024 * 1024;
    let file_max_bytes = req.file_max_size_mb * 1024 * 1024;

    // Generate service names that mimic real microservices
    let service_name_templates = vec![
        "auth-service", "user-service", "order-service", "payment-service",
        "inventory-service", "notification-service", "search-service",
        "analytics-service", "gateway-service", "billing-service",
        "shipping-service", "catalog-service", "review-service",
        "recommendation-service", "email-service", "scheduler-service",
        "config-service", "audit-service", "report-service", "cache-service",
        "media-service", "webhook-service", "export-service", "import-service",
        "monitoring-service", "logging-service", "discovery-service",
        "rate-limiter-service", "session-service", "tenant-service",
    ];

    let service_names: Vec<String> = (0..req.num_services)
        .map(|i| {
            if (i as usize) < service_name_templates.len() {
                service_name_templates[i as usize].to_string()
            } else {
                format!("microservice-{}", i + 1)
            }
        })
        .collect();

    let config = GeneratorConfig {
        num_services: req.num_services,
        target_bytes,
        file_max_bytes,
        output_dir: PathBuf::from("logs"),
        service_names,
    };

    start_generation(config, Arc::clone(&state));

    Ok(Json(StartResponse {
        message: format!(
            "Started generating {} MB of logs across {} services",
            req.total_size_mb, req.num_services
        ),
    }))
}

async fn stop_handler(State(state): State<AppState>) -> Json<StartResponse> {
    state.cancel.store(true, Ordering::SeqCst);
    Json(StartResponse {
        message: "Stop signal sent. Generation will halt shortly.".into(),
    })
}

async fn progress_handler(State(state): State<AppState>) -> Json<ProgressResponse> {
    let bytes_written = state.bytes_written.load(Ordering::Relaxed);
    let target = state.target_bytes.load(Ordering::Relaxed);
    let percent = if target > 0 {
        (bytes_written as f64 / target as f64) * 100.0
    } else {
        0.0
    };

    Json(ProgressResponse {
        running: state.running.load(Ordering::SeqCst),
        bytes_written,
        target_bytes: target,
        percent: (percent * 100.0).round() / 100.0, // 2 decimal places
        services_total: state.services_total.load(Ordering::SeqCst),
        services_done: state.services_done.load(Ordering::SeqCst),
    })
}

