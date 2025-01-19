use crate::diesel::Connection;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(db_connection_string: String, with_migration: bool) -> () {
    if with_migration {
        let mut connection = PgConnection::establish(&db_connection_string)
            .expect("Failed to create DB for migrations.");

        match connection.run_pending_migrations(MIGRATIONS) {
            Ok(_) => {
                tracing::event!(target: "[MIGRATIONS]", tracing::Level::INFO, "Migrations applied");
            }
            Err(e) => {
                tracing::event!(target: "[MIGRATIONS]", tracing::Level::ERROR, "Failed to apply migrations, {}", e);
            }
        };
    };
}
