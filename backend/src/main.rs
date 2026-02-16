use axum::{
    routing::{get, post},
    Router,
    extract::Json,
    response::IntoResponse,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

async fn index() -> impl IntoResponse {
    "Welcome to Kitso's Portfolio Backend!"
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
    // Serve static files from the frontend directory
    let serve_dir = ServeDir::new("frontend")
        .append_index_html_on_directories(true);

    let app = Router::new()
        .route("/api", get(index))
        .route("/api/contact", post(handle_contact))
        .fallback_service(serve_dir);

    // Railway provides PORT via environment variable
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Bind to 0.0.0.0 so Railway can access it
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("🚀 Server listening on http://{}", addr);
    println!("📂 Serving frontend from: ./frontend");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
        
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
