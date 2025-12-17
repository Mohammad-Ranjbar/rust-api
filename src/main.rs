use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tower_http::trace::TraceLayer;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with middleware
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/shopw", get(shopw_handler))
        .layer(TraceLayer::new_for_http());  // افزودن logging

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    tracing::info!("🚀 Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Json<Message> {
    tracing::info!("Root handler called");
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}

async fn shopw_handler() -> Json<Message> {
    tracing::info!("Shopw handler called");
    Json(Message {
        message: "Hello from shopw route!".to_string(),
    })
}