mod config;
mod clients;
mod grpc;
mod error;
mod routes;
mod dtos;

use summer::{auto_config, App};

use summer_web::{
    WebConfigurator, WebPlugin,
};

// Main function entry
#[auto_config(WebConfigurator)] // auto config web router
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(WebPlugin)
        .run()
        .await
}