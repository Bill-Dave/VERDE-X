use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub totp_secret: Option<String>,
    pub kyc_status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Escrow {
    pub id: Uuid,
    pub asset: String,
    pub amount: sqlx::types::BigDecimal,
    pub initiator_id: Uuid,
    pub counterparty_id: Uuid,
    pub status: String,
    pub btc_txid: Option<String>,
    pub sol_sig: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct DisputeMessage {
    pub id: i32,
    pub escrow_id: Uuid,
    pub user_id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
}
