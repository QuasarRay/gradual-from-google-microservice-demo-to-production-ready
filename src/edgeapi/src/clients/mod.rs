// src/edge-api-rs/src/clients/mod.rs
use anyhow::Context;
use summer::component;
use tonic::transport::{Channel, Endpoint};

use crate::config::BackendConfig;
use crate::grpc::hipstershop::{
    ad_service_client::AdServiceClient,
    cart_service_client::CartServiceClient,
    checkout_service_client::CheckoutServiceClient,
    currency_service_client::CurrencyServiceClient,
    product_catalog_service_client::ProductCatalogServiceClient,
    recommendation_service_client::RecommendationServiceClient,
    shipping_service_client::ShippingServiceClient,
};

#[derive(Clone)]
pub struct BackendClients {
    pub product_catalog: ProductCatalogServiceClient<Channel>,
    pub currency: CurrencyServiceClient<Channel>,
    pub cart: CartServiceClient<Channel>,
    pub recommendation: RecommendationServiceClient<Channel>,
    pub checkout: CheckoutServiceClient<Channel>,
    pub shipping: ShippingServiceClient<Channel>,
    pub ad: AdServiceClient<Channel>,
    pub default_currency: String,
}

async fn connect(addr: &str) -> anyhow::Result<Channel> {
    Endpoint::from_shared(addr.to_string())
        .with_context(|| format!("invalid gRPC endpoint: {addr}"))?
        .connect()
        .await
        .with_context(|| format!("failed to connect to gRPC endpoint: {addr}"))
}

impl BackendClients {
    pub async fn from_config(config: BackendConfig) -> anyhow::Result<Self> {
        let product_catalog =
            ProductCatalogServiceClient::new(connect(&config.product_catalog_addr).await?);

        let currency =
            CurrencyServiceClient::new(connect(&config.currency_addr).await?);

        let cart =
            CartServiceClient::new(connect(&config.cart_addr).await?);

        let recommendation =
            RecommendationServiceClient::new(connect(&config.recommendation_addr).await?);

        let checkout =
            CheckoutServiceClient::new(connect(&config.checkout_addr).await?);

        let shipping =
            ShippingServiceClient::new(connect(&config.shipping_addr).await?);

        let ad =
            AdServiceClient::new(connect(&config.ad_addr).await?);

        Ok(Self {
            product_catalog,
            currency,
            cart,
            recommendation,
            checkout,
            shipping,
            ad,
            default_currency: config.default_currency,
        })
    }
}