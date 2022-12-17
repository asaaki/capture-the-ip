pub(crate) mod api;
pub(crate) mod helpers;
mod router;
pub(crate) mod r#static;

#[cfg(debug_assertions)]
pub(crate) mod debug;

pub(crate) use router::router;
