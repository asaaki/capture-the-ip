use crate::{bg, prelude::*};
use futures::FutureExt;
use std::time::Duration;
use tokio::select;
use tokio_graceful_shutdown::SubsystemHandle;

pub(crate) struct BackgroundWorkerSubSystem {
    pub(crate) pool: DbPool,
}

impl BackgroundWorkerSubSystem {
    pub(crate) async fn run(self, subsys: SubsystemHandle) -> AppResult {
        let (bg_tx, stop_rx) = bg::background_thread(self.pool);

        let pending = std::future::pending::<()>();
        select! {
            _ = subsys.on_shutdown_requested() => {
                bg::terminate(bg_tx);
                tokio::time::timeout(Duration::from_secs(5), stop_rx.fuse()).await??;
            },
            _ = pending => {},
        }
        Ok(())
    }
}
