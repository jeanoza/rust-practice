use chrono::NaiveDateTime;
use diesel::Insertable;
use serde_derive::Serialize;

use crate::schema::articles;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    pub title: String,
    pub content: String,
    pub user_id: i32,
	pub created_at:NaiveDateTime
}
