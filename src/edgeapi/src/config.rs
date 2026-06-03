// src/edge-api-rs/src/config.rs
use serde::Deserialize;
use summer::config::Configurable;

#[derive(Debug, Clone, Deserialize, Configurable)]
#[config_prefix = "backend"]
pub struct BackendConfig {
    pub product_catalog_addr: String,
    pub currency_addr: String,
    pub cart_addr: String,
    pub recommendation_addr: String,
    pub checkout_addr: String,
    pub shipping_addr: String,
    pub ad_addr: String,
    pub default_currency: String,
}