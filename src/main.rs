#![allow(unused)]

use actix_web::{App, HttpServer};
mod home;
use actix_files as fs;

use diesel::{r2d2::ConnectionManager, SqliteConnection};
use rand::Rng;

#[macro_use]
extern crate diesel;

mod schema;
mod tables;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8082;
    // use the rng varient if we end up actually running multiple instances at the same time
    // let port = rand::thread_rng().gen_range(50000..60000);

    println!("http://server.zpparel.com:{}", port);

    let connspec = "clothes.db";
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(home::home)
            .service(home::home_like)
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .show_files_listing(),
            )
            .service(
                fs::Files::new("/images", "./data/fashion-dataset/images")
            )
    })
    .bind("0.0.0.0:".to_string() + &port.to_string())?
    .run()
    .await
}
