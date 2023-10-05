use axum::{extract::State, response::IntoResponse, Json, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::users_db::{User, UsersDb};

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub username: String,
}

pub async fn create_user(
    State(users_db): State<UsersDb>,
    Json(create_user_dto): Json<CreateUserDto>,
) -> impl IntoResponse {
    let user = User {
        id: Uuid::new_v4(),
        name: create_user_dto.name,
        username: create_user_dto.username,
    };

    users_db.write().unwrap().insert(user.id, user.clone());

    (StatusCode::CREATED, Json(user))
}
