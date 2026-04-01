use axum::{
    Json, Router,
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
        .route("/users", post(create_user).get(list_users))
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

/* ENDPOINT HANDLERS */
// test endpoint
async fn root() -> &'static str {
    // static lifetime
    "Server up & running"
}

// get all users
async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    // return type: {[Users]} on success & REST StatusCode on error
    sqlx::query_as::<_, User>("SELECT * FROM users") // query
        .fetch_all(&pool) // getting all users
        .await // async fetch
        .map(Json) // Result<T> success
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) // Result<T> error
}

// create user
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserPayload>, // JSON payload for POST req
) -> Result<(StatusCode, Json<User>), StatusCode> {
    sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
        .bind(payload.name) // binding payload attributes
        .bind(payload.email)
        .fetch_one(&pool)
        .await // fetching the created user from db
        .map(|u| (StatusCode::CREATED, Json(u))) // returnging User{} JSON & Status code: 201
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// get user by id
async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>, // to get id from url params
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

// update user
async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *")
        .bind(payload.name)
        .bind(payload.email)
        .bind(id) // binding in the order of $n defined inside the query
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// delete user
async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 { // checking if deleted
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
