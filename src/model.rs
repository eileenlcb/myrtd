use chrono::*;
use std::{
    error::Error,
    fmt::Display,
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};

#[derive(Debug, Clone)]
pub struct Item {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) completed: bool,
    pub(crate) deleted: bool,
    pub(crate) created_at: Option<i64>,
    pub(crate) completed_at: Option<i64>,
    pub(crate) deleted_at: Option<i64>,
}

impl Item {
    pub fn new(
        id:u32,
        name:&str,
        completed:bool,
        deleted:bool,
        created_at:Option<i64>,
        completed_at:Option<i64>,
        deleted_at:Option<i64>,
    ) -> Self{
        Item { id, name: name.to_string(), completed, deleted, created_at, completed_at, deleted_at}
    }
}

const ITEM_COUNT: usize = 7;
const COMMA_FAKE: &str = "<@^_fake_comma_$#>";
const NEWLINE_FAKE: &str = "<@^_fake_newline_$#>";
/// Serialization
impl ToString for Item {
    fn to_string(&self) -> String {
        let created_at = timestamp_to_raw_string(self.created_at);
        let completed_at = timestamp_to_raw_string(self.completed_at);
        let deleted_at = timestamp_to_raw_string(self.deleted_at);

        let name = self
            .name
            .replace(',', COMMA_FAKE)
            .replace(r"\n", NEWLINE_FAKE);

        format!(
            "{},{},{},{},{},{},{}",
            self.id, name, self.completed, self.deleted, created_at, completed_at, deleted_at,
        )
    }
}


fn timestamp_to_raw_string(timestamp: Option<i64>) -> String {
    if let Some(x) = timestamp {
        x.to_string()
    } else {
        String::new()
    }
}