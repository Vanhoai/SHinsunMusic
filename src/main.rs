use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use adapters::shared::{di, firebase};
use core::{configs::app_config::APP_CONFIG, cryptography::keypair};

pub mod adapters;
pub mod application;
pub mod core;
pub mod server;
pub mod state;

#[tokio::main]
async fn main() {
    //  Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    tracing::info!("Starting application");

    // Make sure to load the .env file
    dotenv::dotenv().ok();

    // init dependency injection
    let state = di::build_di().await;
    if state.is_err() {
        tracing::error!("Failed to initialize DI");
        std::process::exit(1);
    }

    // init firebase
    if let Err(e) = firebase::firebase_impl::initial().await {
        tracing::error!("Failed to initialize firebase: {}", e);
        std::process::exit(1);
    }

    // Generate key pair
    if let Err(e) = keypair::generate_key_pair() {
        tracing::error!("Failed to generate key pair: {}", e);
        std::process::exit(1);
    }

    //  Add cors layer to the application
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    //  Add tracing layer to the application
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    //  Add the application to the router
    let app = server::init_router(state.unwrap())
        .layer(cors)
        .layer(trace_layer);

    //  Start the server
    let address = format!(
        "{}:{}",
        APP_CONFIG.server.network.host, APP_CONFIG.server.network.port
    );
    tracing::info!("listening on {} 🎉", address);

    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
