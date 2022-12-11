use crate::types::AppResult;

use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(not(feature = "tracing"))]
pub(crate) fn start_tracing() -> AppResult {
    Ok(())
}

#[cfg(feature = "tracing")]
pub(crate) fn start_tracing() -> AppResult {
    // let fmt_layer = tracing_subscriber::fmt::layer().compact();
    // let fmt_layer = tracing_subscriber::fmt::layer().without_time();
    let fmt_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer)
        .try_init()?;
    Ok(())
}
