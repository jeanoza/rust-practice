use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde_derive::Deserialize;

use crate::{
    db_utils::AppState,
    messages::{CreateArticle, GetArticles, GetUsers},
    DbActor,
};

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    // "Get /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(GetUsers).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No user founded"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[get("/users/{id}/articles")]
pub async fn get_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let db = state.as_ref().db.clone();

    match db.send(GetArticles { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles founded by {}", id)),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve articles"),
    }
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateArticleBody>,
) -> impl Responder {
    let id = path.into_inner();
    let db = state.as_ref().db.clone();

    match db
        .send(CreateArticle {
            user_id: id,
            title: body.title.to_string(),
            content: body.content.to_string(),
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Created().json(info),
        Ok(Err(error)) => HttpResponse::BadRequest().json(format!("error:{}", error.to_string())),
        _ => HttpResponse::InternalServerError().json("Fail to create article"),
    }
}
