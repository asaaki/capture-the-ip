use crate::{app, prelude::*};
use axum::http::header::{HeaderMap, HeaderName, HeaderValue, SERVER};
use std::{net::SocketAddr, time::Duration};
use tokio_graceful_shutdown::SubsystemHandle;
use tower_default_headers::DefaultHeadersLayer;
use tower_http::{
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};

const SERVER_NAME: HeaderValue = HeaderValue::from_static("cti");
const H_CTI_MESSAGE_NAME: HeaderName = HeaderName::from_static("cti-message");
const H_CTI_MESSAGE_VAL: HeaderValue = HeaderValue::from_static("hello!");

pub(crate) struct HttpServerSubSystem {
    pub(crate) pool: DbPool,
}

impl HttpServerSubSystem {
    pub(crate) async fn run(self, subsys: SubsystemHandle) -> AppResult {
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_response(DefaultOnResponse::new().include_headers(true));

        let mut default_headers = HeaderMap::new();
        default_headers.insert(SERVER, SERVER_NAME);
        default_headers.insert(H_CTI_MESSAGE_NAME, H_CTI_MESSAGE_VAL);

        let app = app::router(self.pool)
            .layer(CompressionLayer::new())
            .layer(DefaultHeadersLayer::new(default_headers))
            .layer(TimeoutLayer::new(Duration::from_secs(5)))
            .layer(trace_layer);
        let addr = addr();
        let hostname = hostname();

        log::info!("Listening on {addr} ({hostname}) ...");
        axum::Server::bind(&addr)
            .http1_header_read_timeout(Duration::from_secs(5))
            .http1_half_close(true)
            .http2_enable_connect_protocol()
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .with_graceful_shutdown(subsys.on_shutdown_requested())
            .await
            .map_err(Into::into)
    }
}
