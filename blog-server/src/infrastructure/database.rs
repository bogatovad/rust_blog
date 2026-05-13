use sqlx::{PgPool, migrate};
use sqlx::postgres::PgPoolOptions;

// get connection pool.
pub async fn get_pool(db_url: String) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("failed to connect to database")
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // run migrations in database.
    match migrate!().run(pool).await {
        Ok(_) => {
            println!("Migrations completed successfully");
            Ok(())
        }
        Err(e) => {
            println!("Migration failed: {}", e);
            Err(sqlx::Error::Migrate(Box::new(e)))
        }
    }
}