use crate::db_models::{Article, User};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetUsers;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Article>>")]
pub struct GetArticles {
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct CreateArticle {
    pub user_id: i32,
    pub title: String,
    pub content: String,
}
