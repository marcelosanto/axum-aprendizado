use std::net::SocketAddr;

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use axum_aprendizado::users_db::UsersDb;
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let users_db = UsersDb::default();

    let users_api = Router::with_state(users_db).route("/", post(create_user));

    let api = Router::new()
        .nest("/users", users_api)
        .with_state(users_db)
        .fallback(api_fallback);

    let app = Router::new().nest("/api", api);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_user() -> (StatusCode, Json<Value>) {
    let body = json!({
        "id": Uuid::new_v4(),
        "name": "John Doe",
        "username": "johndoe"
    });

    (StatusCode::CREATED, Json(body))
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404,
        "message": "Not found"
    });

    (StatusCode::NOT_FOUND, Json(body))
}
