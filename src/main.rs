use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use adapters::shared::di;
use core::configs::app_config::APP_CONFIG;

pub mod adapters;
pub mod application;
pub mod core;
pub mod server;

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
    if let Err(e) = di::build_di().await {
        tracing::error!("Failed to initialize DI: {}", e);
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
    let app = server::init_router().layer(cors).layer(trace_layer);

    //  Start the server
    let address = format!(
        "{}:{}",
        APP_CONFIG.server.network.host, APP_CONFIG.server.network.port
    );
    tracing::info!("listening on {} 🎉", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// use std::path::PathBuf;
// use yt_dlp::fetcher::deps::Libraries;
// use yt_dlp::Youtube;

// #[tokio::main]
// pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let url = String::from("https://www.youtube.com/watch?v=slkWAKdjtvg");

//     let libraries_dir = PathBuf::from("libs");
//     let output_dir = PathBuf::from("output");

//     let youtube = libraries_dir.join("yt-dlp");
//     let ffmpeg = libraries_dir.join("ffmpeg");

//     let libraries = Libraries::new(youtube, ffmpeg);
//     let fetcher = Youtube::new(libraries, output_dir)?;

//     let thumbnail_path = fetcher
//         .download_thumbnail_from_url(url, "thumbnail.jpg")
//         .await?;

//     println!("Thumbnail saved to: {}", thumbnail_path.display());
//     Ok(())
// }
