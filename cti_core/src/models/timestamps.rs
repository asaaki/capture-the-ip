use std::borrow::Cow;

use crate::prelude::*;
use crate::types::PgConn;
use cti_schema::schema::timestamps;
use cti_types::GenericResult;
use diesel::{prelude::*, upsert::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
pub(crate) struct Timestamp<'a> {
    pub(crate) id: Cow<'a, str>,
    #[serde(with = "time::serde::timestamp")]
    pub(crate) stamped_at: time::OffsetDateTime,
}

impl<'a> Timestamp<'a> {
    pub async fn create(
        conn: &mut PgConn,
        id: Cow<'a, str>,
        stamped_at: time::OffsetDateTime,
    ) -> GenericResult<Timestamp<'a>> {
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
            .map_err(|e| anyhow!("query issue: {e}"))
    }
}
