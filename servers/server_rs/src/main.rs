use axum::extract::Path;
use axum::{response::Json, routing::get, Router};
use crawler_rs::WikipediaCrawler;
use dotenvy::dotenv;
use http::Method;
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let port: String = std::env::var("VITE_BACKEND_PORT")
        .expect("`VITE_BACKEND_PORT` should be set in .env file")
        .to_string();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let app = Router::new()
        .route("/:starting_page", get(get_path))
        .layer(cors);

    let host = ["127.0.0.1", &port].join(":");
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();

    println!("Server running on port: {port}");
    axum::serve(listener, app).await.unwrap();
}

async fn get_path(Path(starting_page): Path<String>) -> Json<Value> {
    let path = WikipediaCrawler::new(&starting_page).crawl().await;
    let result = path.unwrap_or(vec!["Could not reach Kevin Bacon".to_string()]);

    Json(json!({"starting_page": starting_page, "result": result }))
}
