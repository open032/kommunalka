// use crate::controller::is_cookie;
// use axum::response::{Html, IntoResponse};
// use axum::routing::get;
// use axum::Router;
// use tower_cookies::Cookies;
// use axum::response::{Html, IntoResponse};
pub use self::error::{Error, Result};
use axum::Form;
// use serde::Deserialize;
use crate::ctx::Ctx;
use crate::model::ModelController;
use tower_cookies::Cookie;

use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router};
use serde::Deserialize;
use serde_json::json;
// use tokio::net::TcpListener;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use uuid::Uuid;

use tokio::net::TcpListener;
// use axum::response::{Html, IntoResponse};
// use axum::routing::get;
// use axum::Router;

mod ctx;
mod error;
mod model;
mod web;

mod view;

#[tokio::main]
async fn main() -> Result<()> {
    let routes_all = Router::new()
        .merge(routes_hello().await)
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    Ok(())
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
async fn routes_hello() -> Router {
    Router::new().route("/", get(handler_hello))
    // .route("/tg", get(handler_tg))
}
async fn handler_hello(cookies: Cookies, Query(params): Query<HelloParams>) -> impl IntoResponse {
    let mut cookie = Cookie::new("five", "min");
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.same_site();
    cookies.add(cookie);
    // let s = crate::view::auth_login::test_js().await.to_string();
    Html(crate::view::auth_login::test_js().await.to_string())
    // Html(format!("Hello <strong></strong>"))
}
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
