// src/edge-api-rs/src/routes/products.rs
use summer_web::{
    axum::Json,
    extractor::{Component, Path},
    get_api,
};

use crate::clients::BackendClients;
use crate::dtos::productDto::ProductDto;
use crate::error::ApiError;
use crate::grpc::hipstershop::{Empty, GetProductRequest};

#[get_api("/api/v1/products")]
pub async fn list_products(
    Component(clients): Component<BackendClients>,
) -> Result<Json<Vec<ProductDto>>, ApiError> {
    let mut client = clients.product_catalog.clone();

    let response = client
        .list_products(tonic::Request::new(Empty {}))
        .await?
        .into_inner();

    let products = response
        .products
        .into_iter()
        .map(ProductDto::from)
        .collect();

    Ok(Json(products))
}

#[get_api("/api/v1/products/{id}")]
pub async fn get_product(
    Component(clients): Component<BackendClients>,
    Path(id): Path<String>,
) -> Result<Json<ProductDto>, ApiError> {
    if id.trim().is_empty() {
        return Err(ApiError::BadRequest("product id is required".to_string()));
    }

    let mut client = clients.product_catalog.clone();

    let response = client
        .get_product(tonic::Request::new(GetProductRequest { id }))
        .await?
        .into_inner();

    Ok(Json(ProductDto::from(response)))
}