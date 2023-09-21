#![allow(unused)]
mod model;
mod service;
mod storage;

use clap::{Parser, ValueEnum};
#[derive(Parser, Debug)]
struct Args{
    #[arg(short, long, value_name="item-name")]
    add: Option<String>,
}

enum ListType{
    All,
    Completed,
    Uncompleted,
    Deleted,
}

fn main(){
    let args = Args::parse();
    
    if let Some(name) = args.add{
        match service::add_item(&name){
            Ok(s) => println!("Added item: {}", s),
            Err(e) => println!("Error: {}", e),
        }
    }
}