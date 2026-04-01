use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
}; // backend framework
use serde::{Deserialize, Serialize}; // like mongoose serializer
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions}; // Postgres
use std::env; // for .env

#[derive(Deserialize)]
struct UserPayload {
    name: String,
    email: String,
}

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main] // to make main async
async fn main() {
    let db_url = env::var("DB_URL").expect("DB_URL must be set"); // .expect() acts like try-catch
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB"); // connecting to DB

    sqlx::migrate!().run(&pool).await.expect("Migration failed"); // using migrate!() macro to look into /migrations & run the migration .sql queries (to set up the DB Relations)

    // route defn.
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(creat_user).get(list_users))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(pool);

    // setting up the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap(); // binding to global ip & port
    println!("Server running on PORT: 8000");
    axum::serve(listener, app).await.unwrap(); // analogous: app.listen()
}
