use super::helpers::*;
use crate::{extractors::ClientIpV4, prelude::*};
use axum::extract::ConnectInfo;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use tracing::instrument;

#[instrument]
pub(crate) async fn request_info(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    client_ip: ClientIpV4,
    req: axum::http::Request<axum::body::Body>,
) -> Result<String, (StatusCode, String)> {
    let mut output = String::from("Request Info\n\n");
    output.push_str(&format!("SocketAddr = {addr:?}\n\n"));
    output.push_str(&format!("ClientIpV4 = {client_ip:?}\n\n"));
    output.push_str("Request headers:\n");
    for (k, v) in req.headers() {
        output.push_str(&format!("{k} = {v:?}\n"));
    }
    Ok(output)
}

// Note: use this carefully,
#[instrument]
pub(crate) async fn seed_handler(State(pool): QState) -> Result<Json<bool>, (StatusCode, String)> {
    let items: Vec<NewCapture<'_>> = (0..100)
        .map(|_| {
            let ip: Ipv4Addr = fakedata_generator::gen_ipv4().parse().unwrap();
            let nick = fakedata_generator::gen_username();
            NewCapture::create_from_ip_and_nick_now(ip, nick.into())
        })
        .collect();

    let mut conn = pool.get().await.map_err(internal_error)?;
    Capture::create_many(&mut conn, &items)
        .await
        .map_err(internal_error)?;

    Ok(Json(true))
}
