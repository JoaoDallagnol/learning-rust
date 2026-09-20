#![allow(unused)] // Keep unused warnings quiet while learning.

use crate::ctx::Ctx;
use crate::log::log_request;
use crate::model::ModelController;

pub use self::error::{Error, Result};

use std::fmt::format;

use axum::{
    Json, Router,
    extract::{Path, Query},
    handler::HandlerWithoutStateExt as _,
    http::{Method, StatusCode, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the shared application state.
    let mc = ModelController::new().await?;

    // Protect all ticket routes with the auth middleware.
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // Build the full router tree.
    let routes = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .map_err(|err| format!("Cannot start TcpListener. \nCause: {err}"))?;

    println!("LISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, routes)
        .await
        .map_err(|err| format!("Cannot start axum::serve. \nCause: {err}"))?;

    Ok(())
}

async fn main_response_mapper(
    ctx: Result<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    // Convert the extractor result into an optional request context.
    let ctx = ctx.ok();
    let uuid = Uuid::new_v4();

    // Read the service error stored by Error::into_response.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("   ->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.map(|(_, client_error)| client_error);
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();

    error_response.unwrap_or(res)
}

fn routes_static() -> axum::routing::MethodRouter {
    // Return a simple 404 when the static file is missing.
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Resource not found.")
    }

    get_service(ServeDir::new("./").not_found_service(handle_404.into_service()))
}

fn routes_hello() -> Router {
    // Register small public routes used for learning extractors.
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}
