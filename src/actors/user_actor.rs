use super::DbActor;
use crate::actix::{Handler, Message};
use crate::diesel::prelude::*;
use crate::models::user_models::{CreateUser, User};
use crate::schema::user::dsl::{email, hash, id, name, user};

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct Create {
    pub email: String,
    pub name: String,
    pub text_pass: String,
}

impl Handler<Create> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Unable to get a connectio");
        let create_user = CreateUser {
            email: msg.email,
            name: msg.name,
            hash: msg.text_pass, // TODO: HASH STRONG
        };

        match diesel::insert_into(user)
            .values(create_user)
            .execute(&mut conn)
        {
            Ok(result) => {
                return user
                    .filter(email.like(create_user.email))
                    .get_result::<User>(&mut conn);
            }
            Err(error) => return Err(error),
        }
    }
}
