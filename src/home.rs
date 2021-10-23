use actix_web::{get, post, Responder};
use askama::Template;
use diesel::SqliteConnection;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    images: Vec<String>,
}

#[get("/")]
async fn home(db: &SqliteConnection) -> impl Responder {
    HomeTemplate {
        images: vec![
            "https://placeimg.com/600/300/arch".to_string(),
            "https://placeimg.com/600/300/tech".to_string(),
            "https://placeimg.com/600/300/nature".to_string(),
        ],
    }
}

#[post("/")]
async fn home_like(db: &SqliteConnection) -> impl Responder {
    HomeTemplate {
        images: vec!["test".to_string()]
    }
}