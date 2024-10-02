pub mod mw_auth;
pub mod routes_tickets;

use dotenv::dotenv;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

// use crate::web::model::CookieNameUuid;

pub const AUTH_TOKEN: &str = "auth-token";
pub const ACCEPT_TOKEN: &str = "acp";

pub async fn domain() -> String {
    dotenv().ok();
    std::env::var("DOMAIN").expect("DOMAIN must be set.")
}

pub async fn login() -> String {
    dotenv().ok();
    std::env::var("LOGIN").expect("LOGIN must be set.")
}

pub async fn check_login_pass(log: &str, password: &str) -> bool {
    let mut res = false;
    let mut num = 0;
    dotenv().ok();
    let name = std::env::var("AA").expect("NAME must be set.");
    let dd = std::env::var("DD").expect("NAME must be set.");
    let pass = std::env::var("PASS").expect("NAME must be set.");
    let pass2 = std::env::var("PASS2").expect("NAME must be set.");
    let v_log = vec![name.to_string(), dd.to_string()];
    let v_pass = vec![pass.to_string(), pass2.to_string()];
    for word in v_log {
        if word == log {
            if v_pass[num] == password {
                res = true
            }
        }
        num += 1;
    }
    res
}
pub async fn return_login_cookie(s: &str) -> String {
    let mut res = "".to_string();
    if s == "Don" {
        res = "pravo".to_string();
    }
    if s == "Arch" {
        res = "smotrit".to_string();
    }
    res
}

pub async fn email() -> String {
    dotenv().ok();
    std::env::var("EMAIL").expect("DOMAIN must be set.")
}
pub async fn check_cookie(cookie: Cookies) -> bool {
    let mut res = false;

    dotenv().ok();
    let name_aa = std::env::var("AA").expect("NAME must be set.");
    let _cookie_aa = match cookie.get(&name_aa).map(|c| c.value().to_string()) {
        None => "None".to_string(),
        Some(c) => {
            res = true;
            c
        }
    };

    dotenv().ok();
    let name = std::env::var("DD").expect("NAME must be set.");
    let _cookie_get = match cookie.get(&name).map(|c| c.value().to_string()) {
        None => "None".to_string(),
        Some(c) => {
            res = true;
            c
        }
    };
    // println!("AA {:?}", _cookie_aa);
    res
}
// pub async fn get_name_cookie(cookie: Cookies) -> CookieNameUuid {
//     let uuid = Uuid::now_v7().to_string();
//     let mut cookie_name = "".to_string();
//     dotenv().ok();
//     let name_aa = std::env::var("AA").expect("NAME must be set.");
//     let _cookie_aa = match cookie.get(&name_aa).map(|c| c.value().to_string()) {
//         None => "None".to_string(),
//         Some(c) => {
//             cookie_name = c.clone();
//             crate::pg_db::pass_uuid::update_pass(&cookie_name, &uuid).await;
//             c
//         }
//     };
//     let name = std::env::var("DD").expect("NAME must be set.");
//     let _cookie_get = match cookie.get(&name).map(|c| c.value().to_string()) {
//         None => "None".to_string(),
//         Some(c) => {
//             cookie_name = c.clone();
//             crate::pg_db::pass_uuid::update_pass(&cookie_name, &uuid).await;
//             c
//         }
//     };
//     let _cookie_five = match cookie.get(&"five").map(|c| c.value().to_string()) {
//         None => "None".to_string(),
//         Some(c) => {
//             cookie_name = c.clone();
//             crate::pg_db::pass_uuid::update_pass(&cookie_name, &uuid).await;
//             c
//         }
//     };
//     CookieNameUuid {
//         name: cookie_name,
//         uuid: uuid,
//     }
// }
