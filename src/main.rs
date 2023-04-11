extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod actors;
mod models;
mod schema;
mod connection_pool;

use actix_web::{
    delete, get, patch, post, put,
    web::{self, Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};

use actix::SyncArbiter;
use actors::DbActor;
use connection_pool::mysql_pool::{run_migrations, get_pool};
use models::{AppState};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service()
            .data(AppState {
                db: db_addr.clone(),
            })
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
