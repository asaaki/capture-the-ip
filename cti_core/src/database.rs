use crate::prelude::*;
use deadpool_diesel::{PoolConfig, Timeouts};
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::RunQueryDsl;
use futures::FutureExt;
use once_cell::sync::OnceCell;
use rustls::{RootCertStore, ClientConfig};
use std::time::Duration;

pub(crate) use cti_schema::*;

static TLS_CONFIG: OnceCell<ClientConfig> = OnceCell::new();

pub(crate) async fn setup_db() -> GenericResult<DbPool> {
    let database_url = if crate::utils::is_home() {
        db_admin_connection_str()
    } else {
        db_connection_str()
    };
    let conn_url = url::Url::parse(&database_url).expect("invalid DB URL");
    let db_scheme = conn_url.scheme();
    let db_host = conn_url.host_str().unwrap_or("(unknown)");
    let db_port = conn_url
        .port_or_known_default()
        .unwrap_or(db_default_port_for(db_scheme));
    let db_user = conn_url.username();
    let db_origin = format!("{db_scheme}://{db_user}@{db_host}:{db_port}");

    let max_size = db_pool_size();
    let timeouts = Timeouts {
        wait: Some(Duration::from_secs(5)),
        create: Some(Duration::from_secs(5)),
        recycle: Some(Duration::from_secs(5)),
    };
    let config = PoolConfig { max_size, timeouts };

    TLS_CONFIG
        .set(tls_config())
        .map_err(|_| eyre!("client config issue"))?;

    let manager = Manager::new_with_setup(database_url, |url| establish(url).boxed());
    let pool = DbPool::builder(manager)
        .config(config)
        .runtime(deadpool::Runtime::Tokio1)
        .build()?;

    log::info!("Connect to {db_origin} with pool size {max_size} ...");

    let mut conn = pool.get().await?;
    diesel::sql_query("select true as app_connected")
        .execute(&mut conn)
        .await?;

    Ok(pool)
}

fn db_default_port_for(db_scheme: &str) -> u16 {
    match db_scheme {
        "postgres" => 5432,
        "mysql" => 3306,
        _ => 0,
    }
}

async fn establish(database_url: &str) -> ConnectionResult<PgConn> {
    let connector =
        tokio_postgres_rustls::MakeRustlsConnect::new(TLS_CONFIG.get().unwrap().clone());

    let (client, connection) = tokio_postgres::connect(&database_url, connector)
        .await
        .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {e}");
        }
    });
    PgConn::try_from(client).await
}

fn tls_config() -> ClientConfig {
    ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store())
        .with_no_client_auth()
}

fn root_store() -> RootCertStore {
    let mut roots = RootCertStore::empty();
    roots.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    roots
}
