use schemars::JsonSchema;
// src/edge-api-rs/src/routes/cart.rs
use summer_web::{
    axum::Json,
    extractor::{Component, Query},
    delete_api, get_api, post_api,
};

use serde::Deserialize;

use crate::clients::BackendClients;
use crate::dtos::cartDto::{AddCartItemRequestDto, CartDto};
use crate::error::ApiError;
use crate::grpc::hipstershop::{
    AddItemRequest, CartItem, EmptyCartRequest, GetCartRequest,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserQuery {
    pub user_id: String,
}

#[get_api("/api/v1/cart")]
pub async fn get_cart(
    Component(clients): Component<BackendClients>,
    Query(query): Query<UserQuery>,
) -> Result<Json<CartDto>, ApiError> {
    let mut client = clients.cart.clone();

    let response = client
        .get_cart(tonic::Request::new(GetCartRequest {
            user_id: query.user_id,
        }))
        .await?
        .into_inner();

    Ok(Json(CartDto::from(response)))
}

#[post_api("/api/v1/cart/items")]
pub async fn add_cart_item(
    Component(clients): Component<BackendClients>,
    Query(query): Query<UserQuery>,
    Json(body): Json<AddCartItemRequestDto>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.quantity <= 0 {
        return Err(ApiError::BadRequest("quantity must be positive".to_string()));
    }

    let mut client = clients.cart.clone();

    client
        .add_item(tonic::Request::new(AddItemRequest {
            user_id: query.user_id,
            item: Some(CartItem {
                product_id: body.product_id,
                quantity: body.quantity,
            }),
        }))
        .await?;

    Ok(Json(serde_json::json!({ "status": "ok" })))
}

#[delete_api("/api/v1/cart")]
pub async fn empty_cart(
    Component(clients): Component<BackendClients>,
    Query(query): Query<UserQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut client = clients.cart.clone();

    client
        .empty_cart(tonic::Request::new(EmptyCartRequest {
            user_id: query.user_id,
        }))
        .await?;

    Ok(Json(serde_json::json!({ "status": "ok" })))
}