use std::env;

use axum::Router;
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod server;
mod git;

/**
 * This repo is used as a test
 */
const GIT_REPO: &str = "https://github.com/lorey/mlscraper";

#[tokio::main]
async fn main() {

    // Clone repository if it doesn't exist

    let _ = git::clone_repo(GIT_REPO);

    let server: &dyn server::Server = &server::StaticServer::create("test_server");
    server.update();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rwdbms=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let server_port = env::var("RWDBMS_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(8080);

    serve(serve_static_dir(), server_port).await;
}

fn serve_static_dir() -> Router {
    let serve_dir = ServeDir::new("static");

    Router::new()
        .fallback_service(serve_dir)
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening on port {addr}");

    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
