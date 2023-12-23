use biodome::biodome;
use cti_constants::*;

pub fn addr() -> std::net::SocketAddr {
    format!("{}:{}", host_ip(), port())
        .parse()
        .expect("HOST_IP:PORT does not form a valid address")
}

pub fn host_ip() -> String {
    biodome("HOST_IP", DEFAULT_IP)
}

pub fn port() -> String {
    biodome("PORT", DEFAULT_PORT)
}

pub fn hostname() -> String {
    biodome("HOST_IP", "NO_HOSTNAME_SET")
}

pub fn db_connection_str() -> String {
    biodome("DATABASE_URL", DEFAULT_DB_URL)
}

pub fn db_admin_connection_str() -> String {
    biodome("DATABASE_ADMIN_URL", db_connection_str())
}

pub fn db_pool_size() -> usize {
    biodome("DATABASE_POOL_SIZE", DEFAULT_DB_POOL_SIZE)
}

// fly.io specifics

pub fn fly_region() -> String {
    biodome("FLY_REGION", "___")
}

// sentry

pub fn sentry_dsn() -> Option<String> {
    std::env::var("SENTRY_DSN").ok()
}
