use crate::{model::Item, storage};
use chrono::Local;
use std::{error::Error, fmt::Display};
use std::io;

pub fn add_item(name: &str) -> Result<String,io::Error> {
    let max_id = 999;
    let item = Item::new(
        max_id + 1,
        name,
        false,
        false,
        Some(Local::now().timestamp()),
        None,
        None,
    );
    storage::add_item(item.clone())?;
    Ok(format!("Added [{}]: {}\n", item.id, item.name))
}