//! Type aliases for using `deadpool` with `diesel_logger`.

// use crate::types::*;

// // use diesel_logger::LoggingConnection;

// /// Manager for PostgreSQL connections
// // pub type Manager = deadpool_diesel::Manager<LoggingConnection<diesel::PgConnection>>;
// // pub(crate) type Manager = diesel_async::pooled_connection::AsyncDieselConnectionManager<LoggingConnection<diesel_async::AsyncPgConnection>>;
// pub use deadpool::managed::reexports::*;
// pub use deadpool_sync::reexports::*;
// deadpool::managed_reexports!(
//     "diesel_tracing",
//     Manager,
//     deadpool::managed::Object<Manager>,
//     diesel::ConnectionError,
//     std::convert::Infallible
// );

// /// Type alias for [`Object`]
// pub type Connection = Object;
