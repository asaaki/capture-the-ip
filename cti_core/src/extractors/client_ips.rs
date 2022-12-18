use axum::{
    async_trait,
    extract::{ConnectInfo, FromRequestParts},
    http::{header::FORWARDED, request::Parts, Extensions, HeaderMap, StatusCode},
};
use forwarded_header_value::{ForwardedHeaderValue, Identifier};
use indexmap::IndexSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::{marker::Sync, net::SocketAddr};
use tracing::instrument;

type V4AndV6 = (IndexSet<Ipv4Addr>, IndexSet<Ipv6Addr>);

const X_REAL_IP: &str = "x-real-ip";
const X_FORWARDED_FOR: &str = "x-forwarded-for";
const FLY_CLIENT_IP: &str = "fly-client-ip";

#[allow(dead_code)] // TODO: maybe remove this extractor
#[derive(Debug)]
pub(crate) struct ClientIps {
    pub(crate) ipv4: IndexSet<Ipv4Addr>,
    pub(crate) ipv6: IndexSet<Ipv6Addr>,
}

#[async_trait]
impl<S> FromRequestParts<S> for ClientIps
where
    S: Sync + std::fmt::Debug,
{
    type Rejection = (StatusCode, &'static str);

    #[instrument]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut ips = V4AndV6::default();
        get_fly_client_ip(&parts.headers, &mut ips);
        get_x_forwarded_for(&parts.headers, &mut ips);
        get_x_real_ip(&parts.headers, &mut ips);
        get_forwarded(&parts.headers, &mut ips);
        get_conn_info(&parts.extensions, &mut ips);
        let (ipv4, ipv6) = ips;
        Ok(Self { ipv4, ipv6 })
    }
}

fn get_fly_client_ip(headers: &HeaderMap, (v4s, v6s): &mut V4AndV6) {
    headers
        .get(FLY_CLIENT_IP)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse::<IpAddr>().ok())
        .map(|addr| match addr {
            IpAddr::V4(v4) => v4s.insert(v4),
            IpAddr::V6(v6) => v6s.insert(v6),
        });
}

fn get_x_forwarded_for(headers: &HeaderMap, (v4s, v6s): &mut V4AndV6) {
    headers
        .get(X_FORWARDED_FOR)
        .and_then(|hv| hv.to_str().ok())
        .map(|s| {
            s.split(',')
                .flat_map(|s| s.trim().parse::<IpAddr>().ok())
                .map(|addr| match addr {
                    IpAddr::V4(v4) => v4s.insert(v4),
                    IpAddr::V6(v6) => v6s.insert(v6),
                })
        });
}

fn get_x_real_ip(headers: &HeaderMap, (v4s, v6s): &mut V4AndV6) {
    headers
        .get(X_REAL_IP)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse::<IpAddr>().ok())
        .map(|addr| match addr {
            IpAddr::V4(v4) => v4s.insert(v4),
            IpAddr::V6(v6) => v6s.insert(v6),
        });
}

fn get_forwarded(headers: &HeaderMap, (v4s, v6s): &mut V4AndV6) {
    headers
        .get_all(FORWARDED)
        .iter()
        .flat_map(|hv| {
            hv.to_str()
                .ok()
                .and_then(|s| ForwardedHeaderValue::from_forwarded(s).ok())
                .map(|f| {
                    f.iter()
                        .filter_map(|fs| fs.forwarded_for.as_ref())
                        .flat_map(|ff| match ff {
                            Identifier::SocketAddr(a) => Some(a.ip()),
                            Identifier::IpAddr(ip) => Some(*ip),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
        })
        .flatten()
        .for_each(|addr| {
            match addr {
                IpAddr::V4(v4) => v4s.insert(v4),
                IpAddr::V6(v6) => v6s.insert(v6),
            };
        });
}

fn get_conn_info(extensions: &Extensions, (v4s, v6s): &mut V4AndV6) {
    extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| match addr.ip() {
            IpAddr::V4(v4) => v4s.insert(v4),
            IpAddr::V6(v6) => v6s.insert(v6),
        });
}
