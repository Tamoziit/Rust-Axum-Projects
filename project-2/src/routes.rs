use axum::{Extension, Router, middleware};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::{
    AppState,
    handler::{auth::auth_handler, users::user_handler},
    middleware::auth,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/auth", auth_handler()) // auth routes
        .nest("/users", user_handler().layer(middleware::from_fn(auth))) // auth protected user routes
        .layer(TraceLayer::new_for_http()) // logging
        .layer(Extension(app_state)); // passing shared app state across all routers

    Router::new().nest("/api", api_route) // base router /api
}
