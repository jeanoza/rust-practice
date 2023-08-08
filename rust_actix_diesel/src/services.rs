use actix_web::{
    get, post,
    web::{Json, Path},
    Responder,
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    "Get /users".to_string()
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
