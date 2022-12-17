pub(crate) mod api;
pub(crate) mod r#static;
pub(crate) mod helpers;
mod router;

#[cfg(debug_assertions)]
pub(crate) mod debug;

pub(crate) use router::router;
