use crate::actors::DbActor;
use crate::actix::Addr;
pub mod user_models;

pub struct AppState {
    pub db: Addr<DbActor>,
}
