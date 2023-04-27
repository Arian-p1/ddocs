use crate::compress::{compress, decompress};
use crate::serdef::*;
use clap::Parser;
use std::io::{self, BufRead};

fn input() -> String {
    let mut ii = String::new();
    io::stdin().lock().read_line(&mut ii).unwrap();
    ii
}

// key is the topic we want to search
pub fn search(key: String) {
    let mut map = json_to_hashmap();
    for k in map.keys() {
        if key.contains(k) {
            println!("Result: {}", key);
        }
    }
}

pub fn add(key: String) {
    let value = input();
    let mut map = json_to_hashmap();
    map.insert(key, compress(&value));
    hashmap_to_json(map);
}

pub fn cat(key: String) {
    let mut map = json_to_hashmap();
    if let Some(value) = map.get(&key) {
        println!("{}", decompress(&value));
    } else {
        println!("there is no topic with this name");
    }
}

// TODO
pub fn edit(key: String) {}

pub fn delete(key: String) {
    let mut map = json_to_hashmap();
    map.remove(&key);
    hashmap_to_json(map);
    println!("the topic is deleted");
}

#[derive(Parser, Debug)]
#[command(version = "0.9")]
#[command(about = r#"

      .o8        .o8
     "888       "888
 .oooo888   .oooo888   .ooooo.   .ooooo.   .oooo.o
d88' `888  d88' `888  d88' `88b d88' `"Y8 d88(  "8
888   888  888   888  888   888 888       `"Y88b.
888   888  888   888  888   888 888   .o8 o.  )88b
`Y8bod88P" `Y8bod88P" `Y8bod8P' `Y8bod8P' 8""888P'


    "#)]
struct Cli {
    /// search the topic you saved
    #[clap(short, long, required(false))]
    search: Option<String>,
    // topic: Option<String>, // key of hashmap
    /// add your new topic
    #[clap(short, long, required(false))]
    add: Option<String>,
    /// cat your topic
    #[clap(short, long, required(false))]
    cat: Option<String>,
    /// edit topic (its on TODO)
    #[clap(short, long, required(false))]
    edit: Option<String>,
    /// delete the topic
    #[clap(short, long, required(false))]
    delete: Option<String>,
}

pub fn run() {
    let args = Cli::parse();
    match args {
        Cli {
            search: Some(val), ..
        } => {
            println!("searching...");
            search(val);
        }
        Cli { add: Some(val), .. } => {
            println!("write your doc");
            add(val);
        }
        Cli { cat: Some(val), .. } => {
            cat(val);
        }
        Cli {
            edit: Some(val), ..
        } => {
            edit(val);
        }
        Cli {
            delete: Some(val), ..
        } => {
            delete(val);
        }

        _ => println!("give me an option \nto see options use -h or --help"),
    }
}
