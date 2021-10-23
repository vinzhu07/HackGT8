use actix_web::{
    error::ErrorBadRequest, get, http::Error, http::StatusCode, post, web, HttpResponse, Responder,
    Result,
};
use askama::Template;
use diesel::prelude::*;
use diesel::{QueryDsl, SqliteConnection, sql_query};
use serde::Deserialize;

use crate::tables::Clothes;
use crate::schema::clothes;
use crate::tables::Swipes;
use crate::DbPool;

#[derive(Template)]
#[template(path = "likes.html")]
struct LikeTemplate {
    items: Vec<Clothes>
}

#[get("/likes")]
async fn likes(pool: web::Data<DbPool>) -> Result<impl Responder> {

    let conn = pool.get().map_err(|e| ErrorBadRequest(e))?;
    let items: Vec<Clothes> = sql_query("SELECT * FROM clothes INNER JOIN swipes Swipes ON clothes.id = Swipes.cloth_id WHERE love_status = 1;")
    .load(&conn)
    .map_err(|e| ErrorBadRequest(e))?;

    Ok(DislikeTemplate {
        items
    })
}

#[derive(Template)]
#[template(path = "dislikes.html")]
struct DislikeTemplate {
    items: Vec<Clothes>
}

#[get("/dislikes")]
async fn dislikes(pool: web::Data<DbPool>) -> Result<impl Responder> {

    let conn = pool.get().map_err(|e| ErrorBadRequest(e))?;
    let items: Vec<Clothes> = sql_query("SELECT * FROM clothes INNER JOIN swipes Swipes ON clothes.id = Swipes.cloth_id WHERE love_status = 0;")
    .load(&conn)
    .map_err(|e| ErrorBadRequest(e))?;

    Ok(DislikeTemplate {
        items
    })
}