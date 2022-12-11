use std::borrow::Cow;

use crate::prelude::*;
use crate::types::PgConn;
use chrono::naive::serde::ts_seconds::serialize as to_ts;
use chrono::{prelude::*, NaiveDateTime};
use cti_schema::schema::timestamps;
use cti_types::GenericResult;
use diesel::{prelude::*, upsert::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
pub(crate) struct Timestamp<'a> {
    pub(crate) id: Cow<'a, str>,
    #[serde(serialize_with = "to_ts")]
    pub(crate) stamped_at: NaiveDateTime,
}

impl<'a> Timestamp<'a> {
    pub async fn create(
        conn: &mut PgConn,
        id: Cow<'a, str>,
        ts: DateTime<Utc>,
    ) -> GenericResult<Timestamp<'a>> {
        let stamped_at =
            chrono::NaiveDateTime::from_timestamp_opt(ts.timestamp(), ts.timestamp_subsec_nanos())
                .unwrap();
        let value = Self { id, stamped_at };

        diesel::insert_into(timestamps::table)
            .values(&value)
            .on_conflict(timestamps::id)
            .do_update()
            .set((
                timestamps::id.eq(excluded(timestamps::id)),
                timestamps::stamped_at.eq(excluded(timestamps::stamped_at)),
            ))
            .get_result(conn)
            .await
            .map_err(|e| eyre!("query issue: {e}"))
    }
}
