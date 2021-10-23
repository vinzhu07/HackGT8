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
#[template(path = "likes.html")]
struct LikeTemplate {
    items: Vec<Clothes>
}

#[get("/likes")]
async fn likes(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use crate::schema::clothes::dsl::*;

    let conn = pool.get().map_err(|e| ErrorBadRequest(e))?;
    let items = clothes
        .load::<Clothes>(&conn)
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
    use crate::schema::clothes::dsl::*;
    //use crate::schema::swipes::dsl::*;

    let conn = pool.get().map_err(|e| ErrorBadRequest(e))?;
    let items = clothes
        .load::<Clothes>(&conn)
        .inner_join(swipes.on(cloth_id.eq(id)))
        .filter()
        .map_err(|e| ErrorBadRequest(e))?;

    Ok(DislikeTemplate {
        items
    })
}