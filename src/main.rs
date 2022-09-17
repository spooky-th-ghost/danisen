use axum::{response::Html, routing::get, Router};
use dotenv;
use std::net::SocketAddr;

mod types;

pub use types::*;

#[tokio::main]
async fn main() {
    println!("{}", dotenv::var("BLAH").unwrap());
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
