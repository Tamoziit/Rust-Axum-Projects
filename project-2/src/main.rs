mod config;
mod db;
mod dtos;
mod errors;
mod models;
mod utils;
mod middleware;

use axum::{
    Extension,
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
};
use config::Config;
use db::DBClient;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[tokio::main] // tokio async runtime
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init(); // for tracing server logs

    dotenv().ok(); // loading env

    let config = Config::init(); // reading from env & storing in Config struct

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to PostgresDB");
            pool
        }
        Err(err) => {
            println!("Error in connecting to DB: {:?}", err);
            std::process::exit(1)
        }
    }; // DB connection

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]); // CORS configuration

    let db_client = DBClient::new(pool);

    let app_state = AppState {
        env: config.clone(),
        db_client,
    };
    let app = Router::new()
        .layer(Extension(app_state))
        .layer(cors.clone()); // app init [analogous app = expresS()]

    println!("{}", format!("Server is running on PORT: {}", config.port));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap(); // listening incoming HTTP/TCP requests on IP: 0.0.0.0:PORT

    axum::serve(listener, app).await.unwrap();
}
