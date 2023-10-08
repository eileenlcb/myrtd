use crate::{model::Item, storage};
use chrono::Local;
use std::{error::Error, fmt::Display};
use std::io;

pub fn add_item(name: &str) -> Result<String,io::Error> {
    let max_id = storage::get_max_id()?;
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

pub fn complete_item(id: u32) -> Result<String,io::Error> {
    let item = storage::get_item_by_id(id)?;
    storage::update_item(Item {
        completed: true,
        completed_at: Some(Local::now().timestamp()),
        ..item.clone()
    })?;
    Ok(format!("Completed [{}]: {}\n", item.id, item.name))
}