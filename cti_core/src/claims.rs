use crate::prelude::*;
use futures::future::FutureExt;
use itertools::Itertools;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::{
    sync::oneshot::{channel, Receiver, Sender},
    time::{interval_at, Duration, Instant},
};

const REFRESH_INTERVAL: Duration = Duration::from_secs(1);

type ClaimsQueue = Arc<Mutex<Vec<NewCapture>>>;

pub(crate) struct TerminationToken;

pub(crate) fn background_thread(
    pool: DbPool,
    claims_receiver: ClaimsReceiver,
) -> (Sender<TerminationToken>, Receiver<TerminationToken>) {
    let (bg_tx, bg_rx) = channel();
    let (stop_tx, stop_rx) = channel();

    std::thread::spawn(move || background_runtime(bg_rx, stop_tx, pool, claims_receiver));
    (bg_tx, stop_rx)
}

pub(super) fn terminate(bg_tx: Sender<TerminationToken>) {
    bg_tx
        .send(TerminationToken)
        .unwrap_or_else(|_| log::error!("error on sending terminal token"));
}

#[tokio::main(flavor = "current_thread")]
pub(crate) async fn background_runtime(
    bg_rx: Receiver<TerminationToken>,
    stop_tx: Sender<TerminationToken>,
    pool: DbPool,
    claims_receiver: ClaimsReceiver,
) -> AppResult {
    log::info!("Claims thread and runtime started.");
    let mut bg_rx = bg_rx.fuse();
    let start = Instant::now() + REFRESH_INTERVAL;
    let mut interval = interval_at(start, REFRESH_INTERVAL);
    let mut stop_tx = Some(stop_tx);
    let mut claims_receiver = claims_receiver;
    let queue = ClaimsQueue::default();
    let (sink, drain) = (queue.clone(), queue.clone());

    loop {
        tokio::select! {
            res = &mut bg_rx => {
                log::info!("±±± Claims thread terminating ...");
                let token = res.unwrap_or_else(|e| {
                    log::error!("±±± Receiver error: {e}");
                    TerminationToken
                });

                if let Some(tx) = stop_tx.take() {
                    tx.send(token).unwrap_or_else(|_| {
                            log::error!("±±± Could not send TerminationToken back to main thread");
                        });
                } else {
                    log::error!("±±± TerminationToken sender consumed already");
                }

                log::info!("±±± Claims thread terminated.");
                break Ok(()); // SUPER IMPORTANT!
            }
            r = claims_receiver.recv() => {
                if let Some(claim) = r {
                    sink.lock().push(claim);
                }
            }
            _ = interval.tick() => {
                handler(&pool, &drain).await.unwrap_or_else(|e| {
                    log::error!("±±± Error on claim writing [report only] {e}");
                });
            }
        }
    }
}

async fn handler(pool: &DbPool, drain: &ClaimsQueue) -> AppResult {
    let mut conn = pool.get().await?;
    conn.build_transaction()
        .serializable()
        .deferrable()
        .run(|conn| {
            Box::pin(async move {
                let mut claims = {
                    let mut drain = drain.lock();
                    std::mem::take(&mut *drain)
                };
                let drain_len = claims.len();
                let drain_cap = claims.capacity();

                let claims = claims
                    .drain(..)
                    .rev()
                    .unique_by(|c| c.ip)
                    .collect::<Vec<_>>();

                if !claims.is_empty() {
                    log::info!(
                        "±±± Writing {} claim(s) to DB; drain vec: {}/{}",
                        claims.len(),
                        drain_len,
                        drain_cap
                    );
                    let result = Capture::create_many(conn, &claims).await;
                    // to not lose claims on error, reinsert them into drain
                    if let Err(e) = result {
                        log::error!("±±± Error on claim writing: {e}");
                        log::info!("±±± Reinserting {} claim(s) into drain", claims.len());
                        let mut claims = claims;
                        // hold the lock for as short as possible
                        {
                            let mut drain = drain.lock();
                            drain.extend(claims.drain(..).rev());
                        }
                    }
                }
                Ok(())
            })
        })
        .await
}
