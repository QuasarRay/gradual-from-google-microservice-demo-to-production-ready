use schemars::JsonSchema;
// src/edge-api-rs/src/routes/checkout.rs
use serde::{Deserialize, Serialize};
use summer_web::{axum::Json, extractor::Component, post_api};

use crate::clients::BackendClients;
use crate::dtos::productDto::MoneyDto;
use crate::error::ApiError;
use crate::grpc::hipstershop::{
    Address, CreditCardInfo, PlaceOrderRequest,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckoutRequestDto {
    pub user_id: String,
    pub email: String,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip_code: i32,

    // temporary compatibility with existing demo proto
    pub credit_card_number: String,
    pub credit_card_cvv: i32,
    pub credit_card_expiration_year: i32,
    pub credit_card_expiration_month: i32,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct CheckoutResponseDto {
    pub order_id: Option<String>,
    pub shipping_tracking_id: Option<String>,
    pub shipping_cost: Option<MoneyDto>,
}

#[post_api("/api/v1/checkout")]
pub async fn checkout(
    Component(clients): Component<BackendClients>,
    Json(body): Json<CheckoutRequestDto>,
) -> Result<Json<CheckoutResponseDto>, ApiError> {
    let mut client = clients.checkout.clone();

    let response = client
        .place_order(tonic::Request::new(PlaceOrderRequest {
            user_id: body.user_id,
            user_currency: clients.default_currency.clone(),
            address: Some(Address {
                street_address: body.street_address,
                city: body.city,
                state: body.state,
                country: body.country,
                zip_code: body.zip_code,
            }),
            email: body.email,
            credit_card: Some(CreditCardInfo {
                credit_card_number: body.credit_card_number,
                credit_card_cvv: body.credit_card_cvv,
                credit_card_expiration_year: body.credit_card_expiration_year,
                credit_card_expiration_month: body.credit_card_expiration_month,
            }),
        }))
        .await?
        .into_inner();

    let order = response.order;

    let body = CheckoutResponseDto {
        order_id: order.as_ref().map(|o| o.order_id.clone()),
        shipping_tracking_id: order.as_ref().map(|o| o.shipping_tracking_id.clone()),
        shipping_cost: order
            .and_then(|o| o.shipping_cost)
            .map(MoneyDto::from),
    };

    Ok(Json(body))
}