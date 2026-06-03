use std::env;
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
impl BackendConfig {
    pub fn from_env() -> Self {
        Self {
            product_catalog_addr: env_or(
                "PRODUCT_CATALOG_SERVICE_ADDR",
                "http://productcatalogservice:3550",
            ),
            currency_addr: env_or(
                "CURRENCY_SERVICE_ADDR",
                "http://currencyservice:7000",
            ),
            cart_addr: env_or(
                "CART_SERVICE_ADDR",
                "http://cartservice:7070",
            ),
            recommendation_addr: env_or(
                "RECOMMENDATION_SERVICE_ADDR",
                "http://recommendationservice:8080",
            ),
            checkout_addr: env_or(
                "CHECKOUT_SERVICE_ADDR",
                "http://checkoutservice:5050",
            ),
            shipping_addr: env_or(
                "SHIPPING_SERVICE_ADDR",
                "http://shippingservice:50051",
            ),
            ad_addr: env_or(
                "AD_SERVICE_ADDR",
                "http://adservice:9555",
            ),
            default_currency: env_or("DEFAULT_CURRENCY", "USD"),
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}