use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub(crate) struct Ranking {
    pub(crate) nick: String,
    pub(crate) blocks: Vec<i16>,
    pub(crate) total_claims: i64,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub(crate) struct UserRanking {
    #[serde(rename = "block")]
    pub(crate) blck: i16,
    pub(crate) rank: i32,
    pub(crate) max_rank: i32,
    pub(crate) is_tied: bool,
    pub(crate) claims: i64,
    pub(crate) nick: String,
}

#[derive(Debug, Deserialize, Serialize, QueryableByName)]
pub(crate) struct RefreshView {}
