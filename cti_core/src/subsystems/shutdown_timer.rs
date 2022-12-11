use crate::prelude::*;
use std::{thread::sleep, time::Duration};
use tokio::select;
use tokio_graceful_shutdown::SubsystemHandle;

pub(crate) struct ShutdownTimerSubSystem {
    pub(crate) seconds: u64,
}

impl ShutdownTimerSubSystem {
    pub(crate) async fn run(self, subsys: SubsystemHandle) -> AppResult {
        let pending = std::future::pending::<()>();
        select! {
            _ = subsys.on_shutdown_requested() => {
                std::thread::spawn(move || countdown(self.seconds));
            },
            _ = pending => {},
        }
        Ok(())
    }
}

pub(crate) fn countdown(seconds: u64) {
    for i in (1..=seconds).rev() {
        log::warn!("Forced shutdown in {} ...", i);
        sleep(Duration::from_secs(1));
    }
}
