pub mod user_models;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub struct DatabaseConnection(
    pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);
