use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub(crate) mod sql;

async fn root() -> &'static str {
    "hello"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
