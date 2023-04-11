use crate::actix::{Actor, SyncContext};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};
pub struct DbActor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

pub mod user_actor;
