
use actix::SyncArbiter;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use std::env;


mod services;
mod db_utils;
mod db_models;
mod messages;

use services::{create_user_article, get_user_articles, get_users};
use db_utils::{AppState, DbActor,get_pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be defined in .env");
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(AppState {db:db_addr.clone()})
            .service(create_user_article)
            .service(get_user_articles)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
