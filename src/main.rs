#![allow(unused)]

use clap::Parser;

mod commands;
mod compress;
mod serdef;

#[derive(Parser, Debug)]
#[command(name = "ddcos")]
#[command(author = "arian ahmadi <ahmadiarian981@gmail.com>")]
#[command(version = "0.9")]
#[command(about = "save docs wit ddocs and feel relief because i compress your data so you can enjoy from more space ", long_about = None)]
struct Cli {
    /// search the topic you saved
    #[clap(short, long, required(false))]
    search: String,
    // topic: String, // key of hashmap
    /// add your new topic
    #[clap(short, long, required(false))]
    add: String,
    /// cat your topic
    #[clap(short, long, required(false))]
    cat: String,
    /// edit topic
    #[clap(short, long, required(false))]
    edit: String,
    /// delete the topic
    #[clap(short, long, required(false))]
    delete: String,
}

/*
    now we open json file and put it into hashmap
    for better performance in the future i make a header file
    and split files if they are more than 10mg
    then make the hashmap of array[keys]
*/
fn main() {
    let args = Cli::parse();

    match args.search.as_str() {
        "-s" => println!("somthing"),
        "-S" => println!("somthing"),
        "-a" => println!("somthing"),
        "-o" => println!("somthing"),
        "-e" => println!("soon"),
        "-d" => println!("somthing"),
        _ => println!("hmmm"),
    }
}
