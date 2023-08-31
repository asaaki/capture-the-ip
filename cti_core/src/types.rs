pub use cti_types::*;

use crate::models::NewCapture;
use axum::extract::State;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub(crate) type PgConn = AsyncPgConnection;
pub(crate) type DbPool = Pool<PgConn>;
pub(crate) type Manager = AsyncDieselConnectionManager<PgConn>;

pub(crate) type ClaimsSender = UnboundedSender<NewCapture>;
pub(crate) type ClaimsReceiver = UnboundedReceiver<NewCapture>;

pub(crate) type AppState = (DbPool, ClaimsSender);
pub(crate) type QState = State<AppState>;
