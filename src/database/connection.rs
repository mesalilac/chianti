use diesel::{RunQueryDsl, SqliteConnection, r2d2};
use std::path::Path;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
struct EnableForeignKeys;

impl r2d2::CustomizeConnection<SqliteConnection, r2d2::Error> for EnableForeignKeys {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), r2d2::Error> {
        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(conn)
            .map(|_| ())
            .map_err(diesel::r2d2::Error::QueryError)
    }
}

fn get_database_url() -> String {
    if cfg!(debug_assertions) {
        tracing::debug!("Using in-memory database");
        "dev.db".to_string()
    } else {
        let db_path = Path::new("/app/data");
        if !db_path.exists() {
            match std::fs::create_dir_all(db_path) {
                Ok(_) => {
                    tracing::debug!("Created database directory");
                }
                Err(e) => {
                    tracing::error!("Failed to create database directory: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // Inside docker container
        "/app/data/chianti.db".to_string()
    }
}

pub fn create_connection_pool() -> DbPool {
    let url = get_database_url();
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    let pool = r2d2::Pool::builder()
        .test_on_check_out(true)
        .connection_customizer(Box::new(EnableForeignKeys))
        .build(manager)
        .expect("Could not build connection pool");

    let mut conn = pool.get().unwrap();

    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut conn)
        .expect("Failed to enable foreign_keys");

    pool
}
