use diesel::{
    backend,
    r2d2::{ConnectionManager, Pool},
    Connection, MysqlConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(db_url: &str) {
    let mut conn = MysqlConnection::establish(db_url).unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
