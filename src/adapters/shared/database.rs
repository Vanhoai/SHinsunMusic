use mongodb::{options::ClientOptions, Client, Database};

use crate::core::configs::app_config::APP_CONFIG;

pub async fn init_database() -> Database {
    let url = APP_CONFIG.database.url.clone();
    let name = APP_CONFIG.database.name.clone();

    let client_options = ClientOptions::parse(url).await.unwrap();

    // Create a new client and connect to the server
    let client = Client::with_options(client_options).unwrap();
    client.database(&name)
}
