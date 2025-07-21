use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateEscrow {
    asset: String,
    amount: f64,
    counterparty_email: String,
    duration_hours: i32,
}

#[derive(Serialize)]
struct EscrowRes { id: Uuid }

async fn create_escrow(
    State(pool): State<PgPool>,
    Json(body): Json<CreateEscrow>,
) -> Result<Json<EscrowRes>, StatusCode> {
    let counter = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        body.counterparty_email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::BAD_REQUEST)?;
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO escrows (id, asset, amount, initiator_id, counterparty_id, expires_at)
        VALUES ($1, $2, $3, $4, $5, now() + ($6 || ' hours')::interval)
        "#,
        id,
        body.asset,
        body.amount,
        Uuid::nil(), // TODO real user
        counter.id,
        body.duration_hours as i64
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(EscrowRes { id }))
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_escrow))
        .route("/:id", get(|| async { "escrow detail" }))
}
