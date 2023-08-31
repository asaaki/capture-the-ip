use std::time::Duration;
use subsystems::*;
use tokio_graceful_shutdown::Toplevel;
use types::*;

mod app;
mod bg;
mod claims;
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

const GRACE_PERIOD_SECONDS: u64 = 15;

#[allow(unsafe_code)]
// SAFETY: we control both ends; also no_mangle is considered unsafe(?)
#[no_mangle]
pub extern "C" fn run(mode: RunMode) -> u8 {
    match run_async(mode) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("CORE ERR: {e}");
            1
        }
    }
}

fn run_async(mode: RunMode) -> AppResult {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime build issue")
        .block_on(run_app(mode))?;
    Ok(())
}

async fn run_app(mode: RunMode) -> AppResult {
    utils::setup()?;

    #[cfg(debug_assertions)]
    debug::list_env_vars();

    let pool: deadpool_diesel::Pool<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    > = database::setup_db().await?;
    let (claims_sender, claims_receiver) = tokio::sync::mpsc::unbounded_channel();

    let http_subsystem = http_server::HttpServerSubSystem {
        pool: pool.clone(),
        sender: claims_sender,
    };
    let cw_subsystem = claim_writer::ClaimWriterSubSystem {
        pool: pool.clone(),
        receiver: claims_receiver,
    };
    let bg_subsystem = background::BackgroundWorkerSubSystem { pool: pool.clone() };
    let st_subsystem = shutdown_timer::ShutdownTimerSubSystem {
        seconds: GRACE_PERIOD_SECONDS,
    };

    let mut toplevel = Toplevel::new().start("ST", |subsys| st_subsystem.run(subsys));

    match mode {
        RunMode::Api => {
            toplevel = toplevel
                .start("CLAIMS", |subsys| cw_subsystem.run(subsys))
                .start("API", |subsys| http_subsystem.run(subsys));
        }
        RunMode::Background => {
            toplevel = toplevel.start("BG", |subsys| bg_subsystem.run(subsys));
        }
        RunMode::Dual => {
            toplevel = toplevel
                .start("BG", |subsys| bg_subsystem.run(subsys))
                .start("CLAIMS", |subsys| cw_subsystem.run(subsys))
                .start("API", |subsys| http_subsystem.run(subsys));
        }
    }

    toplevel
        .catch_signals()
        .handle_shutdown_requests(Duration::from_secs(GRACE_PERIOD_SECONDS))
        .await?;

    log::info!("Have a nice day!");
    Ok(())
}
