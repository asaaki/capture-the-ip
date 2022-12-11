#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

use cti_env::*;
use cti_types::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

fn main() -> AnyResult {
    let mut connection = establish_connection();
    run_migrations(&mut connection)?;
    Ok(())
}

fn run_migrations(connection: &mut impl MigrationHarness<diesel::pg::Pg>) -> AnyResult {
    let mut harness = HarnessWithOutput::write_to_stdout(connection);
    if harness.has_pending_migration(MIGRATIONS)? {
        harness.run_pending_migrations(MIGRATIONS)?;
    } else {
        println!("No pending migrations.");
    }
    Ok(())
}

pub fn establish_connection() -> PgConnection {
    let database_url = db_admin_connection_str();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
