use std::{env, net::SocketAddr};

use axum::{
    middleware,
    response::{IntoResponse, Response},
    Json, Router, Server,
};
use dotenv::dotenv;
use serde_json::json;
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
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    info!("Listening on {addr}");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

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
