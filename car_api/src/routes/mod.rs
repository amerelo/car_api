pub mod note;
pub mod user;

use sqlx::postgres::PgPool;

/// The data that is shared across the processes.
pub struct AppState {
    pub pg_pool: PgPool,
}
