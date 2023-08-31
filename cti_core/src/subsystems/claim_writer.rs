use crate::{claims, prelude::*};
use futures::FutureExt;
use std::time::Duration;
use tokio::select;
use tokio_graceful_shutdown::SubsystemHandle;

pub(crate) struct ClaimWriterSubSystem {
    pub(crate) pool: DbPool,
    pub(crate) receiver: ClaimsReceiver,
}

impl ClaimWriterSubSystem {
    pub(crate) async fn run(self, subsys: SubsystemHandle) -> AppResult {
        let (cw_tx, stop_rx) = claims::background_thread(self.pool, self.receiver);

        let pending = std::future::pending::<()>();
        select! {
            _ = subsys.on_shutdown_requested() => {
                claims::terminate(cw_tx);
                tokio::time::timeout(Duration::from_secs(5), stop_rx.fuse()).await??;
            },
            _ = pending => {},
        }
        Ok(())
    }
}
