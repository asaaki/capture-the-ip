use crate::prelude::*;
use deadpool_diesel::{PoolConfig, Timeouts};
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::{pooled_connection::ManagerConfig, RunQueryDsl};
use rustls::{pki_types::CertificateDer, ClientConfig, RootCertStore};
use std::{sync::OnceLock, time::Duration};

pub(crate) use cti_schema::*;

static TLS_CONFIG: OnceLock<ClientConfig> = OnceLock::new();

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
        wait: Some(Duration::from_secs(15)),
        create: Some(Duration::from_secs(15)),
        recycle: Some(Duration::from_secs(15)),
    };
    let config = PoolConfig {
        max_size,
        timeouts,
        ..Default::default()
    };

    TLS_CONFIG
        .set(tls_config())
        .map_err(|e| anyhow!("client config issue; E={e:?}"))?;

    let mut manager_config = ManagerConfig::default();
    manager_config.custom_setup = Box::new(|conn| Box::pin(establish(conn)));
    let manager = Manager::new_with_config(database_url, manager_config);
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

    let (client, connection) = tokio_postgres::connect(database_url, connector)
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
        .with_root_certificates(root_store())
        .with_no_client_auth()
}

fn root_store() -> RootCertStore {
    let mut roots = RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    roots.add_parsable_certificates(aws_rds_roots());
    roots
}

fn aws_rds_roots() -> Vec<CertificateDer<'static>> {
    let aws_certs = include_bytes!("../../certs/aws-global-bundle.pem");
    let mut aws_certs = std::io::BufReader::new(aws_certs.as_slice());
    rustls_pemfile::read_all(&mut aws_certs)
        .filter_map(|item| {
            if let Ok(rustls_pemfile::Item::X509Certificate(bytes)) = item {
                Some(bytes)
            } else {
                None
            }
        })
        .collect()
}
