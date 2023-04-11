use diesel::{
    connection::Connection,
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};

pub fn run_migrations(db_url: &str) {
    embed_migrations!();
    let connection = MysqlConnection::establish(db_url).expect("Error connecting to database");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Error running migrations");
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
