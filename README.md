use axum::{
    routing::{get, post},
    extract::State,
    Json, Router,
};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use serde::{Serialize, Deserialize};
use tokio;

#[derive(Serialize, Deserialize)]
struct UserDto {
    name: String,
    email: String,
}

// SeaORM entity example
mod entity;
use entity::user::Entity as User;

#[tokio::main]
async fn main() {
    let db = Database::connect("sqlite::memory:").await.unwrap();

    let app = Router::new()
        .route("/users", get(get_users).post(create_user))
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_users(
    State(db): State<DatabaseConnection>,
) -> Json<Vec<entity::user::Model>> {
    let users = User::find().all(&db).await.unwrap();
    Json(users)
}

async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<UserDto>,
) -> Json<String> {
    // insert logic later
    Json(format!("Created user: {}", payload.name))
}
