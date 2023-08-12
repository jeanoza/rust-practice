use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde_derive::Deserialize;

use crate::{db_utils::AppState, messages::GetUsersMsg, DbActor};

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    // "Get /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(GetUsersMsg).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No user founded"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[get("/users/{id}/articles")]
pub async fn get_user_articles(path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    format!("Get /users/{id}/articles")
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(path: Path<i32>, body: Json<CreateArticleBody>) -> impl Responder {
    let id = path.into_inner();
    format!("Post /users/{id}/articles")
}
