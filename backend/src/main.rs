use axum::{
    routing::{get, post},
    Router,
    extract::Json,
    response::IntoResponse,
};
use serde::Deserialize;
use std::{net::SocketAddr};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

async fn index() -> impl IntoResponse {
    "Welcome to Kitso’s Portfolio Backend!"
}

async fn handle_contact(Json(payload): Json<ContactForm>) -> impl IntoResponse {
    println!("New message from: {} <{}>", payload.name, payload.email);
    println!("Message: {}", payload.message);

    axum::Json(serde_json::json!({
        "status": "success",
        "message": "Thanks for reaching out, Kitso will get back to you soon!"
    }))
}

#[tokio::main]
async fn main() {
    // Get frontend path from environment or use relative fallback
    let frontend_path = std::env::var("FRONTEND_PATH")
        .unwrap_or_else(|_| "../frontend".to_string());
    
    let serve_dir = ServeDir::new(&frontend_path)
        .append_index_html_on_directories(true);

    let app = Router::new()
        .route("/api", get(index))
        .route("/api/contact", post(handle_contact))
        .fallback_service(serve_dir);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{}", addr);
    println!("Serving frontend from: {}", frontend_path);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
