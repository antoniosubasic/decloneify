use axum::{routing::post, Extension, Json, Router};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Deserialize)]
struct Payload {
    username: String,
    password: String,
}

#[axum::debug_handler]
async fn sign_in(pool: Extension<Arc<PgPool>>, Json(payload): Json<Payload>) -> StatusCode {
    let pool = pool.0.as_ref();
    let record = sqlx::query!("SELECT * FROM users WHERE username = $1", payload.username)
        .fetch_one(pool)
        .await;

    match record {
        Ok(user) => match user.password == payload.password {
            true => StatusCode::OK,
            false => StatusCode::UNAUTHORIZED,
        },
        Err(_) => {
            sqlx::query!(
                "INSERT INTO users (username, password) VALUES ($1, $2)",
                payload.username,
                payload.password
            )
            .execute(pool)
            .await
            .unwrap();

            StatusCode::CREATED
        }
    }
}

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/signin", post(sign_in))
        .layer(Extension(pool))
}
