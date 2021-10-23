use actix_web::{HttpResponse, Responder, get, http::Error, post, web};
use askama::Template;
use diesel::SqliteConnection;

use crate::DbPool;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    images: Vec<String>,
}

#[get("/")]
async fn home(pool: web::Data<DbPool>) -> Result<impl Responder, Error> {
    let conn = pool.get()?;

    Ok(HomeTemplate {
        images: vec![
            "https://placeimg.com/600/300/arch".to_string(),
            "https://placeimg.com/600/300/tech".to_string(),
            "https://placeimg.com/600/300/nature".to_string(),
        ],
    })
}

#[post("/")]
async fn home_like(pool: web::Data<DbPool>) -> impl Responder {
    HomeTemplate {
        images: vec!["test".to_string()]
    }
}