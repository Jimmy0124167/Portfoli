use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json as AxumJson, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing_subscriber;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize)]
struct Project {
    id: &'static str,
    title: &'static str,
    description: &'static str,
    status: &'static str,
    tags: &'static [&'static str],
}

#[derive(Debug, Deserialize)]
struct ContactPayload {
    name: String,
    email: String,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init logging
    tracing_subscriber::fmt::init();

    // CORS: allow all origins for development. Lock down in production.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Serve ./frontend folder as static files
    let static_files = ServeDir::new("frontend").append_index_html_on_directories(true);

    let app = Router::new()
        // API endpoints
        .route("/api/projects", get(api_projects))
        .route("/api/contact", post(api_contact))
        // Serve static files (fallback)
        .fallback_service(static_files)
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

async fn api_projects() -> impl IntoResponse {
    let projects = vec![
        Project {
            id: "thread",
            title: "Thread",
            description: "Organizational communication app: lightweight threads, channels, and privacy-minded defaults.",
            status: "in development",
            tags: &["web", "communication", "react/planned"],
        },
        Project {
            id: "rust-2d-game",
            title: "2D Rust Game",
            description: "Learning project to explore Rust's ownership model, game loops, and rendering (ggez/wgpu).",
            status: "prototype",
            tags: &["rust", "game", "2d"],
        },
        Project {
            id: "attachment",
            title: "Industrial Attachment (Technology Affiliates)",
            description: "Hardware support, router & access point configuration, data cleansing (June–July 2025).",
            status: "completed",
            tags: &["hardware", "networking", "data"],
        },
    ];
    AxumJson(projects)
}

async fn api_contact(Json(payload): Json<ContactPayload>) -> impl IntoResponse {
    // very small, safe persistence: append JSON lines to `contact_submissions.jsonl`
    let serialized = match serde_json::to_string(&payload) {
        Ok(s) => s,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "serialization error"),
    };

    if let Err(e) = append_contact_line("data/contact_submissions.jsonl", serialized).await {
        tracing::error!("Failed to save contact: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "failed to save");
    }

    (StatusCode::CREATED, AxumJson(serde_json::json!({"status":"ok"})))
}

async fn append_contact_line(path: &str, line: String) -> anyhow::Result<()> {
    // ensure folder exists
    if let Some(parent) = std::path::Path::new(path).parent() {
        fs::create_dir_all(parent).await?;
    }

    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    f.write_all(line.as_bytes()).await?;
    f.write_all(b"\n").await?;
    Ok(())
}
