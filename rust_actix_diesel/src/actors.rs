use crate::db_models::User;
use crate::db_utils::DbActor;
use crate::messages::GetUsersMsg;
use crate::schema::users::dsl::users;
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<GetUsersMsg> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, msg: GetUsersMsg, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("GetUsersMsg: unable to etablish connection");

        let res = users.load::<User>(&mut conn);
        res
    }
}
