use axum::{
    middleware,
    response::{IntoResponse, Response},
    Json, Router,
};
use dotenv::dotenv;
use serde_json::json;
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{debug, info};

mod shared;

use crate::shared::{app_state::AppState, db::DB, error::Error, error::Result};

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let db = DB::new().await?;

    let state = AppState {
        conn: db.conn().clone(),
    };

    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CorsLayer::very_permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    info!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    return Ok(());
}

async fn main_response_mapper(res: Response) -> Response {
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                }
            });

            debug!("client_error_body: {client_error_body}");

            return (*status_code, Json(client_error_body)).into_response();
        });

    return error_response.unwrap_or(res);
}
