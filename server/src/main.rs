mod db;

use std::net::SocketAddr;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use crate::db::Db;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:short_code", get(handle));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn handle(short_code: Path<String>) -> Result<Redirect, ApiError> {
    let conn = std::env::var("local_db")?;
    let db = Db::connect(&conn).await?;
    let result = db.find_url(&short_code.0).await;
    match result {
        None => Err(ApiError::NotFound),
        Some(r) => Ok(Redirect::permanent(&r)),
    }
}

enum ApiError {
    InternalServerError(anyhow::Error),
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::InternalServerError(e) => {
                eprintln!("{e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error occurred.").into_response()
            }
            ApiError::NotFound => {
                (StatusCode::NOT_FOUND, "Not found").into_response()
            }
        }
    }
}

impl<E> From<E> for ApiError where E: Into<anyhow::Error> {
    fn from(err: E) -> Self {
        Self::InternalServerError(err.into())
    }
}