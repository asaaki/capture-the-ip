use std::time::Duration;
use subsystems::*;
use tokio_graceful_shutdown::Toplevel;
use types::*;

mod app;
mod bg;
mod constants;
mod database;
mod debug;
mod environment;
mod extractors;
mod models;
mod prelude;
mod privdrop;
mod subsystems;
mod tracing;
mod types;
mod utils;

pub mod app_prelude {
    pub use super::run_app;
    pub use super::types::{AppResult, RunMode};
    pub use tokio;
}

pub async fn run_app(mode: RunMode) -> AppResult {
    utils::setup()?;

    #[cfg(debug_assertions)]
    debug::list_env_vars();

    let grace_period = 15;

    let pool = database::setup_db().await?;

    let http_subsystem = http_server::HttpServerSubSystem { pool: pool.clone() };
    let bg_subsystem = background::BackgroundWorkerSubSystem { pool: pool.clone() };
    let st_subsystem = shutdown_timer::ShutdownTimerSubSystem {
        seconds: grace_period,
    };

    let mut toplevel = Toplevel::new().start("ST", |subsys| st_subsystem.run(subsys));

    match mode {
        RunMode::Api => {
            toplevel = toplevel.start("API", |subsys| http_subsystem.run(subsys));
        }
        RunMode::Background => {
            toplevel = toplevel.start("BG", |subsys| bg_subsystem.run(subsys));
        }
        RunMode::Dual => {
            toplevel = toplevel
                .start("BG", |subsys| bg_subsystem.run(subsys))
                .start("API", |subsys| http_subsystem.run(subsys));
        }
    }

    toplevel
        .catch_signals()
        .handle_shutdown_requests(Duration::from_secs(grace_period))
        .await?;

    log::info!("Have a nice day!");
    Ok(())
}
