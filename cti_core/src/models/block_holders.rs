use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub(crate) struct BlockHolders {
    #[serde(rename = "block")]
    pub(crate) blck: i16,
    pub(crate) is_tied: bool,
    pub(crate) nick: String,
    pub(crate) claims: i64,
}
