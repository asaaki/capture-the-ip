use crate::GenericResult;
use crate::{database::schema::captures, prelude::*};
use chrono::naive::serde::ts_seconds::serialize as to_ts;
use chrono::{prelude::*, NaiveDateTime};
use diesel::{prelude::*, upsert::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize, Serializer};
use std::{borrow::Cow, net::Ipv4Addr};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub(crate) struct Capture {
    #[serde(serialize_with = "i32_to_ip")]
    pub(crate) ip: i32,
    #[serde(rename = "block")]
    pub(crate) blck: i16,
    pub(crate) nick: String,
    #[serde(serialize_with = "to_ts")]
    pub(crate) claimed_at: NaiveDateTime,
}

fn i32_to_ip<S: Serializer>(i: &i32, serializer: S) -> Result<S::Ok, S::Error> {
    let ip = Ipv4Addr::from(i.to_be_bytes()).to_string();
    serializer.serialize_str(&ip)
}

impl Capture {
    pub(crate) async fn create_from_ip_and_nick_now(
        conn: &mut PgConn,
        ip: Ipv4Addr,
        nick: &str,
    ) -> GenericResult<Self> {
        Self::create(
            conn,
            NewCapture::create_from_ip_and_nick_now(ip, nick.into()),
        )
        .await
    }

    pub(crate) async fn create(conn: &mut PgConn, item: NewCapture<'_>) -> GenericResult<Self> {
        diesel::insert_into(captures::table)
            .values(&item)
            .on_conflict(captures::ip)
            .do_update()
            .set((
                captures::nick.eq(excluded(captures::nick)),
                captures::claimed_at.eq(excluded(captures::claimed_at)),
            ))
            .get_result(conn)
            .await
            .map_err(|e| eyre!("query issue: {e}"))
    }

    #[allow(dead_code)]
    pub(crate) async fn create_many(
        conn: &mut PgConn,
        items: &[NewCapture<'_>],
    ) -> GenericResult<Self> {
        diesel::insert_into(captures::table)
            .values(items)
            .on_conflict(captures::ip)
            .do_update()
            .set((
                captures::nick.eq(excluded(captures::nick)),
                captures::claimed_at.eq(excluded(captures::claimed_at)),
            ))
            .get_result(conn)
            .await
            .map_err(|e| eyre!("query issue: {e}"))
    }

    pub(crate) fn get_ip(&self) -> Ipv4Addr {
        self.ip.to_be_bytes().into()
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = captures)]
pub(crate) struct NewCapture<'a> {
    pub(crate) ip: i32,
    pub(crate) blck: i16,
    pub(crate) nick: std::borrow::Cow<'a, str>,
    pub(crate) claimed_at: NaiveDateTime,
}

impl<'a> NewCapture<'a> {
    pub(crate) fn create_from_ip_and_nick_now(ip: Ipv4Addr, nick: Cow<'a, str>) -> Self {
        let blck = ip.octets()[0] as i16;
        let ip = i32::from_be_bytes(ip.octets());
        let utc: DateTime<Utc> = Utc::now();
        let claimed_at =
            NaiveDateTime::from_timestamp_opt(utc.timestamp(), utc.timestamp_subsec_nanos())
                .unwrap();

        Self {
            ip,
            blck,
            nick,
            claimed_at,
        }
    }
}
