mod config;
mod clients;
mod grpc;

use summer::{auto_config, App};

use summer_web::{
    WebConfigurator, WebPlugin,
};
use summer_web::{get, route};

// Main function entry
#[auto_config(WebConfigurator)] // auto config web router
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(WebPlugin)
        .run()
        .await
}