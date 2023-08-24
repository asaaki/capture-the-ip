use super::helpers::*;
use crate::{extractors::ClientIpV4, models, prelude::*};
use axum::{
    extract::{Query, State},
    headers::{AccessControlAllowOrigin, HeaderMapExt},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use tracing::instrument;

// REQUEST TYPES (query or body params)

#[derive(Debug, Deserialize)]
pub(crate) struct ClaimRequest {
    name: String,
}

// RESPONSE TYPES

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct GetIpResponse {
    ip: Option<Ipv4Addr>,
    v4: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct RecentClaimsResponse {
    claims: Vec<Capture>,
}

#[derive(Debug, Serialize)]
pub(crate) struct UserRankingResponse {
    last_updated_at: NaiveDateTime,
    rankings: Vec<UserRanking>,
}

#[derive(Debug, Serialize)]
pub(crate) struct BlockHoldersResponse {
    last_updated_at: NaiveDateTime,
    blocks: Vec<BlockHolders>,
}

#[derive(Debug, Serialize)]
pub(crate) struct RankingResponse {
    last_updated_at: NaiveDateTime,
    rankings: Vec<Ranking>,
}

// HANDLERS

#[instrument]
pub(crate) async fn get_ip(
    ClientIpV4 { ip }: ClientIpV4,
) -> Result<Json<GetIpResponse>, (StatusCode, String)> {
    Ok(Json(GetIpResponse {
        ip,
        v4: ip.is_some(),
    }))
}

static CALM_DOWN_PLEASE: &[&str] = &[];

#[instrument(skip(pool))]
pub(crate) async fn claim_ip(
    ClientIpV4 { ip }: ClientIpV4,
    claim_req: Query<ClaimRequest>,
    State(pool): QState,
) -> Result<Response, (StatusCode, String)> {
    if let Some(ip) = ip {
        let mut conn = pool.get().await.map_err(internal_error)?;
        if CALM_DOWN_PLEASE
            .iter()
            .any(|&calm_it| calm_it == claim_req.name)
        {
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                "Calm it down and be fair, please".to_string(),
            ));
        }

        let capture = Capture::create_from_ip_and_nick_now(&mut conn, ip, &claim_req.name)
            .await
            .map_err(internal_error)?;

        let ip = capture.get_ip();
        let (nick, unick) = (&capture.nick, urlencoding::encode(&capture.nick));

        let mut response = Html(indoc::formatdoc!(
            r#"
                <!doctype html>
                <title>The IP {ip:?} was claimed for {nick}.</title>
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link rel="stylesheet" href="/cti.css">
                <p>The IP {ip:?} was claimed for <a href="/users.html?name={unick}">{nick}</a>.
                <p><a href=/>Back to homepage</a>
        "#
        ))
        .into_response();

        // fixes https://github.com/asaaki/capture-the-ip/issues/3
        response
            .headers_mut()
            .typed_insert(AccessControlAllowOrigin::ANY);

        Ok(response)
    } else {
        Err((StatusCode::BAD_REQUEST, "bad request, sorry".to_string()))
    }
}

#[instrument(skip(pool))]
pub(crate) async fn get_recent_claims(
    State(pool): QState,
) -> Result<Json<RecentClaimsResponse>, (StatusCode, String)> {
    use crate::database::schema::captures::dsl::*;
    let mut conn = pool.get().await.map_err(internal_error)?;
    let claims = captures
        .order_by(claimed_at.desc())
        .limit(50)
        .load(&mut conn)
        .await
        .map_err(internal_error)?;

    let response = RecentClaimsResponse { claims };

    Ok(Json(response))
}

#[instrument(skip(pool))]
pub(crate) async fn user_ranking(
    claim_req: Query<ClaimRequest>,
    State(pool): QState,
) -> Result<Json<UserRankingResponse>, (StatusCode, String)> {
    use crate::database::custom_schema::user_ranking::dsl::*;
    let mut conn = pool.get().await.map_err(internal_error)?;
    let rankings = user_ranking
        .filter(nick.eq(&claim_req.name))
        .load(&mut conn)
        .await
        .map_err(internal_error)?;

    use crate::database::schema::timestamps::dsl::*;

    let ts = timestamps
        .filter(id.eq("refresher"))
        .get_result::<models::Timestamp<'_>>(&mut conn)
        .await
        .map_err(internal_error)?;
    let last_updated_at = ts.stamped_at;

    let response = UserRankingResponse {
        last_updated_at,
        rankings,
    };

    Ok(Json(response))
}

#[instrument(skip(pool))]
pub(crate) async fn get_block_holders(
    State(pool): QState,
) -> Result<Json<BlockHoldersResponse>, (StatusCode, String)> {
    use crate::database::custom_schema::block_holders::dsl::*;
    let mut conn = pool.get().await.map_err(internal_error)?;
    let blocks = block_holders
        .limit(256)
        .load(&mut conn)
        .await
        .map_err(internal_error)?;

    use crate::database::schema::timestamps::dsl::*;

    let ts = timestamps
        .filter(id.eq("refresher"))
        .get_result::<models::Timestamp<'_>>(&mut conn)
        .await
        .map_err(internal_error)?;
    let last_updated_at = ts.stamped_at;

    let response = BlockHoldersResponse {
        last_updated_at,
        blocks,
    };

    Ok(Json(response))
}

// TODO: Since we have to use a proc macro within a declarative macro,
//       we can probably replace that with a single macro.
macro_rules! ranking_handler_for {
    ($schema:ident) => {
        // rustc's concat_idents! is not stable yet
        concat_idents::concat_idents!(fn_name = get_, $schema {
        #[instrument(skip(pool))]
        pub(crate) async fn fn_name (
            State(pool): QState,
        ) -> Result<Json<RankingResponse>, (StatusCode, String)> {
            use crate::database::custom_schema::$schema::dsl::*;

            let mut conn = pool.get().await.map_err(internal_error)?;
            let rankings = $schema.load::<Ranking>(&mut conn)
                .await
                .map_err(internal_error)?;

            use crate::database::schema::timestamps::dsl::*;

            let ts = timestamps
                .filter(id.eq("refresher"))
                .get_result::<models::Timestamp<'_>>(&mut conn)
                .await
                .map_err(internal_error)?;
            let last_updated_at = ts.stamped_at;
            let response = RankingResponse {
                last_updated_at,
                rankings,
            };

    Ok(Json(response))
        }
    });
    };
}

ranking_handler_for!(ranking_all_time);
ranking_handler_for!(ranking_year);
ranking_handler_for!(ranking_month);
ranking_handler_for!(ranking_week);
ranking_handler_for!(ranking_day);
ranking_handler_for!(ranking_hour);
