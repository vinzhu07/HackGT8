use actix_web::{App, HttpServer};
mod home;
use actix_files as fs;

use diesel::{SqliteConnection, r2d2::ConnectionManager};
#[allow(unused)]
use rand::Rng;

#[macro_use]
extern crate diesel;

mod tables;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8081;
    // use the rng varient if we end up actually running multiple instances at the same time
    // let port = rand::thread_rng().gen_range(50000..60000);

    println!("http://server.zpparel.com:{}", port);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(|| {
        App::new().data(pool.clone()).service(home::home).service(home::home_like).service(fs::Files::new("/static", "./static").show_files_listing().show_files_listing())
    })
        .bind("0.0.0.0:".to_string() + &port.to_string())?
        .run()
        .await
}
