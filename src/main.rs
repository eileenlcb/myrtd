#![allow(unused)]
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
    println!("{:?}", args);
}