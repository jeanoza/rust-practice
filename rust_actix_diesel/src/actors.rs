use crate::db_models::Article;
use crate::db_models::User;
use crate::db_utils::DbActor;
use crate::insertables::NewArticle;
use crate::messages::{CreateArticle, GetArticles, GetUsers};
use crate::schema::articles::dsl::{
    created_at as article_created_at, deleted_at as article_deleted_at, id as article_id,
    updated_at as article_updated_at, *,
};
use crate::schema::users::dsl::*;
use actix::Handler;
use chrono::Local;
use diesel::insert_into;
use diesel::{self, prelude::*};

impl Handler<GetUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: GetUsers, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("GetUsers: unable to etablish connection");

        users.get_results::<User>(&mut conn)
    }
}

impl Handler<GetArticles> for DbActor {
    type Result = QueryResult<Vec<Article>>;

    fn handle(&mut self, msg: GetArticles, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("GetArticles: unable to etablish connection");

        articles
            .filter(user_id.eq(msg.user_id))
            .get_results::<Article>(&mut conn)
    }
}

impl Handler<CreateArticle> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: CreateArticle, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("CreateArticle: unable to etablish connection");

        let new_article = NewArticle {
            title: msg.title,
            content: msg.content,
            user_id: msg.user_id,
            created_at: Local::now().naive_local(),
        };

        insert_into(articles)
            .values(new_article)
            .returning((
                article_id,
                user_id,
                title,
                content,
                article_created_at,
                article_updated_at.nullable(),
                article_deleted_at.nullable(),
            ))
            .get_result::<Article>(&mut conn)
    }
}
