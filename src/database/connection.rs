use diesel::{RunQueryDsl, SqliteConnection, r2d2};
use std::path::PathBuf;

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

pub fn create_connection_pool(data_path: PathBuf) -> DbPool {
    let url = data_path.join("chianti.db");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(url.to_string_lossy());
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
