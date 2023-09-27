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

#[derive(Debug)]
pub struct ParseItemError(String);

type Result<T> = std::result::Result<T, ParseItemError>;

impl Error for ParseItemError {}

impl Display for ParseItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Deserialization todo fail: {}", self.0)
    }
}

impl From<ParseIntError> for ParseItemError {
    fn from(value: ParseIntError) -> Self {
        Self(value.to_string())
    }
}

impl From<ParseBoolError> for ParseItemError {
    fn from(value: ParseBoolError) -> Self {
        Self(value.to_string())
    }
}


impl FromStr for Item {
    type Err = ParseItemError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let splited = s.split(',').collect::<Vec<_>>();
        if splited.len() != ITEM_COUNT {
            return Err(ParseItemError(format!(
                "Expected {} properties, found {}",
                ITEM_COUNT,
                splited.len()
            )));
        }

        let id = splited[0].parse::<u32>()?;
        let name = &splited[1]
            .replace(COMMA_FAKE, ",")
            .replace(NEWLINE_FAKE, "\n");

        let completed = splited[2].parse::<bool>()?;
        let deleted = splited[3].parse::<bool>()?;

        let created_at = str_to_timestamp(splited[4])?;
        let completed_at = str_to_timestamp(splited[5])?;
        let deleted_at = str_to_timestamp(splited[6])?;

        Ok(Item::new(
            id,
            name,
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
        ))
    }
}


fn str_to_timestamp(s: &str) -> Result<Option<i64>> {
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.parse::<i64>()?))
    }
}