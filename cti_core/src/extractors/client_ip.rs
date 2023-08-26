use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
};
use std::marker::Sync;
use std::net::{IpAddr, Ipv4Addr};
use tracing::instrument;

const FLY_CLIENT_IP: &str = "fly-client-ip";

#[derive(Debug)]
pub(crate) struct ClientIpV4 {
    pub(crate) ip: Ipv4Addr,
}

#[async_trait]
impl<S> FromRequestParts<S> for ClientIpV4
where
    S: Sync,
{
    type Rejection = (StatusCode, &'static str);

    #[instrument(skip(_state))]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if debug_code().as_deref() == parts.headers.get("debug").and_then(|v| v.to_str().ok()) {
            log::info!("@@@ HEADERS\n{:?}", parts.headers);
        }

        // for now this is a fly.io only service
        get_fly_client_ip(&parts.headers).map_or_else(
            || {
                Err((
                    StatusCode::BAD_REQUEST,
                    "Bad Request; only IPv4 addresses are supported",
                ))
            },
            |ip| Ok(ClientIpV4 { ip }),
        )
    }
}

#[instrument]
fn get_fly_client_ip(headers: &HeaderMap) -> Option<Ipv4Addr> {
    headers
        .get(FLY_CLIENT_IP)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse::<IpAddr>().ok())
        .and_then(|addr| match addr {
            IpAddr::V4(v4) => {
                log::debug!("Found fly-client-ip: {v4}");
                Some(v4)
            }
            IpAddr::V6(_) => None,
        })
}

fn debug_code() -> Option<String> {
    std::env::var("DEBUG_CODE").ok()
}
