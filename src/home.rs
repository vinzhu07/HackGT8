use actix_web::{
    error::ErrorBadRequest, get, http::Error, http::StatusCode, post, web, HttpResponse, Responder,
    Result,
};
use askama::Template;
use diesel::prelude::*;
use diesel::{QueryDsl, SqliteConnection};
use serde::Deserialize;

use crate::tables::Clothes;
use crate::tables::Swipes;
use crate::DbPool;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    image: String,
    name: String,
    subtitle: String
}

no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

#[get("/")]
async fn home(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use crate::schema::clothes::dsl::*;

    let conn = pool.get().map_err(|e| ErrorBadRequest(e))?;
    let item = clothes
        .order(RANDOM)
        .first::<Clothes>(&conn)
        .map_err(|e| ErrorBadRequest(e))?;

    Ok(HomeTemplate {
        image: "/images/".to_owned() + &item.id.to_string() + ".jpg",
        name: item.product_display_name,
        subtitle: item.usage
    })
}

#[derive(Deserialize)]
struct SwipeStatus {
    status: bool,
}

#[post("/item_swipe_status")]
async fn home_like(pool: web::Data<DbPool>, swipe_status: web::Json<SwipeStatus>) -> Result<impl Responder> {
    //use crate::schema::clothes::dsl::*;
    use crate::schema::swipes::dsl::*;

    let conn = pool.get().map_err(|e| ErrorBadRequest(e))?;
    //let item = clothes
    //    .order(RANDOM)
    //    .first::<Clothes>(&conn)
    //    .map_err(|e| ErrorBadRequest(e))?;
    swipes.insert;

    println!("{}", swipe_status.status);

    Ok(HttpResponse::Ok())
}
