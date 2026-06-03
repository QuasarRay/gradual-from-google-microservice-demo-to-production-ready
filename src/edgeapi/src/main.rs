// src/edge-api-rs/src/main.rs
mod clients;
mod config;
mod dtos;
mod error;
mod grpc;
mod routes;

use summer::{auto_config, App};
use summer_web::{WebConfigurator, WebPlugin};
use summer_grpc::GrpcPlugin;

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(WebPlugin)
        .add_plugin(GrpcPlugin)
        .run()
        .await
}