use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::PgPool;
use std::sync::Arc;

mod user;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let dburl = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = Arc::new(
        PgPool::connect(&dburl)
            .await
            .expect("failed connecting to postgres"),
    );

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/user", user::routes(pool.clone()));

    let port = std::env::var("PORT").expect("PORT is not set");
    let addr = format!("localhost:{port}");
    println!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
