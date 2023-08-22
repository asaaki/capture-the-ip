use axum::{
    async_trait,
    extract::{ConnectInfo, FromRequestParts},
    http::{header::FORWARDED, request::Parts, Extensions, HeaderMap, StatusCode},
};
use forwarded_header_value::{ForwardedHeaderValue, Identifier};
use std::net::{IpAddr, Ipv4Addr};
use std::{marker::Sync, net::SocketAddr};
use tracing::instrument;

const X_REAL_IP: &str = "x-real-ip";
const X_FORWARDED_FOR: &str = "x-forwarded-for";
const FLY_CLIENT_IP: &str = "fly-client-ip";

#[derive(Debug)]
pub(crate) struct ClientIpV4 {
    pub(crate) ip: Option<Ipv4Addr>,
}

#[async_trait]
impl<S> FromRequestParts<S> for ClientIpV4
where
    S: Sync,
{
    type Rejection = (StatusCode, &'static str);

    #[instrument(skip(_state))]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let ip = get_fly_client_ip(&parts.headers)
            .or_else(|| get_x_forwarded_for(&parts.headers))
            .or_else(|| get_x_real_ip(&parts.headers))
            .or_else(|| get_forwarded(&parts.headers))
            .or_else(|| get_conn_info(&parts.extensions));
        log::debug!("Maybe found an IPv4 addr: {ip:?}");
        Ok(ClientIpV4 { ip })
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

#[instrument]
fn get_x_forwarded_for(headers: &HeaderMap) -> Option<Ipv4Addr> {
    headers
        .get(X_FORWARDED_FOR)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| {
            s.split(',')
                .filter_map(|s| s.trim().parse::<IpAddr>().ok())
                .find_map(|addr| match addr {
                    IpAddr::V4(v4) => {
                        log::debug!("Found x-forwarded-for: {v4}");
                        Some(v4)
                    }
                    IpAddr::V6(_) => None,
                })
        })
}

#[instrument]
fn get_x_real_ip(headers: &HeaderMap) -> Option<Ipv4Addr> {
    headers
        .get(X_REAL_IP)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse::<IpAddr>().ok())
        .and_then(|addr| match addr {
            IpAddr::V4(v4) => {
                log::debug!("Found x-real-ip: {v4}");
                Some(v4)
            }
            IpAddr::V6(_) => None,
        })
}

#[instrument]
fn get_forwarded(headers: &HeaderMap) -> Option<Ipv4Addr> {
    headers
        .get_all(FORWARDED)
        .iter()
        .filter_map(|hv| {
            hv.to_str()
                .ok()
                .and_then(|s| ForwardedHeaderValue::from_forwarded(s).ok())
                .map(|f| {
                    f.iter()
                        .filter_map(|fs| fs.forwarded_for.as_ref())
                        .filter_map(|ff| match ff {
                            Identifier::SocketAddr(a) => Some(a.ip()),
                            Identifier::IpAddr(ip) => Some(*ip),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
        })
        .flatten()
        .find_map(|addr| match addr {
            IpAddr::V4(v4) => {
                log::debug!("Found in 'Forwarded' header: {v4}");
                Some(v4)
            }
            IpAddr::V6(_) => None,
        })
}

#[instrument]
fn get_conn_info(extensions: &Extensions) -> Option<Ipv4Addr> {
    extensions
        .get::<ConnectInfo<SocketAddr>>()
        .and_then(|ConnectInfo(addr)| match addr.ip() {
            IpAddr::V4(v4) => {
                log::debug!("Found in ConnectInfo addr: {v4}");
                Some(v4)
            }
            IpAddr::V6(_) => None,
        })
}
