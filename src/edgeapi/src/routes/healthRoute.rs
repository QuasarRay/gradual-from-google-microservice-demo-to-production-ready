// src/edge-api-rs/src/routes/health.rs
use summer_web::{axum::response::IntoResponse, get};

#[get("/livez")]
pub async fn livez() -> impl IntoResponse {
    "ok"
}

#[get("/readyz")]
pub async fn readyz() -> impl IntoResponse {
    "ok"
}