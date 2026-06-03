use schemars::JsonSchema;
// src/edge-api-rs/src/dto/product.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct MoneyDto {
    pub currency_code: String,
    pub units: i64,
    pub nanos: i32,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ProductDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub picture: String,
    pub categories: Vec<String>,
    pub price: Option<MoneyDto>,
}

// src/edge-api-rs/src/dto/product.rs
use crate::grpc::hipstershop::{Money, Product};

impl From<Money> for MoneyDto {
    fn from(value: Money) -> Self {
        Self {
            currency_code: value.currency_code,
            units: value.units,
            nanos: value.nanos,
        }
    }
}

impl From<Product> for ProductDto {
    fn from(value: Product) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            picture: value.picture,
            categories: value.categories,
            price: value.price_usd.map(MoneyDto::from),
        }
    }
}