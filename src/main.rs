use std::net::SocketAddr;

use axum::routing::Router;

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let api = Router::new();

    let app = Router::new().nest("/api", api);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
