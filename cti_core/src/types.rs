use axum::extract::State;
pub use cti_types::*;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

pub(crate) type PgConn = AsyncPgConnection;
pub(crate) type DbPool = Pool<PgConn>;
pub(crate) type Manager = AsyncDieselConnectionManager<PgConn>;

pub(crate) type AppState = DbPool;
pub(crate) type QState = State<AppState>;
