use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

async fn index() -> Result<impl IntoResponse, StatusCode> {
    Ok("Hello, World!")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    tracing::info!("Starting application");

    dotenv::dotenv().ok();

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

    let app = Router::new()
        .route("/", get(index))
        .layer(cors)
        .layer(trace_layer);
    let address = "127.0.0.1:8080";
    tracing::info!("listening on {} 🎉", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
