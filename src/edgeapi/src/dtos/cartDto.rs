use schemars::JsonSchema;
// src/edge-api-rs/src/dto/cart.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AddCartItemRequestDto {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CartItemDto {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CartDto {
    pub user_id: String,
    pub items: Vec<CartItemDto>,
}

// src/edge-api-rs/src/dto/cart.rs
use crate::grpc::hipstershop::{Cart, CartItem};

impl From<CartItem> for CartItemDto {
    fn from(value: CartItem) -> Self {
        Self {
            product_id: value.product_id,
            quantity: value.quantity,
        }
    }
}

impl From<Cart> for CartDto {
    fn from(value: Cart) -> Self {
        Self {
            user_id: value.user_id,
            items: value.items.into_iter().map(CartItemDto::from).collect(),
        }
    }
}