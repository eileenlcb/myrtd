#![allow(unused)]
mod model;
mod service;
mod storage;

use clap::{Parser, ValueEnum};
#[derive(Parser, Debug)]
struct Args{
    #[arg(short, long, value_name="item-name")]
    add: Option<String>,
    #[arg(short,long,value_name="item-id")]
    complete: Option<u32>,
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

    if let Some(id) = args.complete{
        match service::complete_item(id){
            Ok(s) => println!("Completed item: {}", s),
            Err(e) => println!("Error: {}", e),
        }
    }


}