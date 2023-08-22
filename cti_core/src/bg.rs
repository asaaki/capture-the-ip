use crate::{models, prelude::*};
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use futures::future::FutureExt;
use tokio::{
    sync::oneshot::{channel, Receiver, Sender},
    time::{interval_at, Duration, Instant},
};

const REFRESH_INTERVAL: Duration = Duration::from_secs(15);

const REFRESH_BLOCK_HOLDERS_VIEW: &str = "refresh materialized view block_holders";

const REFRESH_GLOBAL_RANKING_VIEWS: [&str; 6] = [
    "refresh materialized view ranking_hour",
    "refresh materialized view ranking_day",
    "refresh materialized view ranking_week",
    "refresh materialized view ranking_month",
    "refresh materialized view ranking_year",
    "refresh materialized view ranking_all_time",
];

const REFRESH_USER_RANKING_VIEW: &str = "refresh materialized view user_ranking";

pub(crate) struct TerminationToken;

pub(crate) fn background_thread(
    pool: DbPool,
) -> (Sender<TerminationToken>, Receiver<TerminationToken>) {
    let (bg_tx, bg_rx) = channel();
    let (stop_tx, stop_rx) = channel();

    std::thread::spawn(move || {
        background_runtime(bg_rx, stop_tx, pool);
    });
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
) {
    log::info!("Background thread and runtime started.");
    let mut bg_rx = bg_rx.fuse();
    let start = Instant::now() + REFRESH_INTERVAL;
    let mut refresh_timer = interval_at(start, REFRESH_INTERVAL);
    let mut stop_tx = Some(stop_tx);
    loop {
        tokio::select! {
            res = &mut bg_rx => {
                log::info!("Background thread terminating ...");
                let token = res.unwrap_or_else(|e| {
                    log::error!("Receiver error: {e}");
                    TerminationToken
                });

                if let Some(tx) = stop_tx.take() {
                    tx.send(token).unwrap_or_else(|_| {
                            log::error!("Could not send TerminationToken back to main thread");
                        });
                } else {
                    log::error!("TerminationToken sender consumed already");
                }

                log::info!("Background thread terminated.");
                break; // SUPER IMPORTANT!
            }
            _ = refresh_timer.tick() => {
                if crate::utils::is_home() {
                    log::info!("refresher triggered");
                    refresher(&pool).await.unwrap_or_else(|e| {
                        log::error!("Error on Refresh [report only] {e}");
                    });
                }
            }
        }
    }
}

async fn refresher(pool: &DbPool) -> AppResult {
    let mut conn = pool.get().await?;
    conn.build_transaction()
        .serializable()
        .deferrable()
        .run(|conn| {
            Box::pin(async move {
                refresh_block_holders(conn).await?;
                refresh_global_ranks(conn).await?;
                refresh_user_ranks(conn).await?;
                upsert_refreshed_at(conn).await?;
                report_on_max_connections(conn).await?;
                Ok(())
            })
        })
        .await
}

async fn refresh_block_holders(conn: &mut PgConn) -> AppResult {
    diesel::sql_query(REFRESH_BLOCK_HOLDERS_VIEW)
        .execute(conn)
        .await?;
    Ok(())
}

async fn refresh_global_ranks(conn: &mut PgConn) -> AppResult {
    for query in REFRESH_GLOBAL_RANKING_VIEWS {
        diesel::sql_query(query).execute(conn).await?;
    }
    Ok(())
}

async fn refresh_user_ranks(conn: &mut PgConn) -> AppResult {
    diesel::sql_query(REFRESH_USER_RANKING_VIEW)
        .execute(conn)
        .await?;
    Ok(())
}

async fn upsert_refreshed_at(conn: &mut PgConn) -> AppResult {
    models::Timestamp::create(conn, "refresher".into(), Utc::now()).await?;
    Ok(())
}

async fn report_on_max_connections(conn: &mut PgConn) -> AppResult {
    let max = {
        use cti_schema::custom_schema::pg_settings::dsl::*;
        pg_settings
            .select(setting)
            .filter(name.eq("max_connections"))
            .first::<String>(conn)
            .await?
    };

    let current = {
        use cti_schema::custom_schema::pg_stat_activity::dsl::*;
        pg_stat_activity
            .filter(database.eq("neondb"))
            .count()
            .first::<i64>(conn)
            .await?
    };

    Ok(log::info!(
        "current connection info\n* max. connections: {max:>5}\n* open connections: {current:>5}"
    ))
}
