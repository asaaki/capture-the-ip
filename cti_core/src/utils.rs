use crate::prelude::*;
use crate::tracing;

pub(crate) fn setup() -> AppResult {
    color_eyre::install()?;
    crate::privdrop::privdrop();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", cti_constants::RUST_LOG);
    }

    tracing::start_tracing()?;
    Ok(())
}

// Note: for now hardcoded to FRA/DE, that's where current DB lives
pub(crate) fn is_home() -> bool {
    cfg!(debug_assertions) || crate::environment::fly_region() == "fra"
}
