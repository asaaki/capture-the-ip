use crate::types::AppResult;

use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) fn start_tracing() -> AppResult {
    let fmt_layer = tracing_subscriber::fmt::layer().without_time();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(sentry::integrations::tracing::layer())
        .try_init()?;
    Ok(())
}
