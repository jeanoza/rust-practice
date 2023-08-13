use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod actors;
mod db_models;
mod db_utils;
mod insertables;
mod messages;
mod schema;
mod services;

use db_utils::{get_pool, AppState, DbActor};
use services::{create_user_article, get_user_articles, get_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be defined in .env");
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(create_user_article)
            .service(get_user_articles)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
