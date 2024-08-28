#![allow(unused)]

use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use model::ModelController;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;
mod queries;
mod model;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_ticket::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("->> LISTENING ON 0.0.0.0:3000\n");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();

    res
}

// region: --- Routes static

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// endregion: --- Routes static

// region: --- Routes hello

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/name/:name", get(hello_handler2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - hello_params {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!!!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn hello_handler2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - hello_params {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))    
}

// endregion: --- Routes hello
