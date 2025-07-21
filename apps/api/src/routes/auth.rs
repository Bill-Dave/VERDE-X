use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct LoginReq { email: String, password: String }

#[derive(Serialize)]
struct LoginRes { token: String }

async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginReq>,
) -> Result<Json<LoginRes>, StatusCode> {
    if req.email.is_empty() || req.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(LoginRes { token: "fake-jwt".into() }))
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/me", get(|| async { "me" }))
}
