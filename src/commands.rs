use crate::{compress::*, serdef::*};
use clap::Parser;

fn input() -> String {
    let mut ii = String::new();
    std::io::stdin().read_line(&mut ii).unwrap();
    ii
}

// key is the topic we want to search
pub fn search(key: String) {
    let mut map = json_to_hashmap();
    for key in map.keys() {
        if key.contains(key) {
            println!("Result: {}\n", key);
        }
    }
}

pub fn add(key: String) {
    let value = input();
    let mut map = json_to_hashmap();
    map.insert(key, compresse(&value));
    hashmap_to_json(map);
}

pub fn cat(key: String) {
    let mut map = json_to_hashmap();
    if let Some(value) = map.get(&key) {
        println!("{}", decompress(value.to_owned()));
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

///ddcos
///author arian ahmadi <ahmadiarian981@gmail.com>
///save docs wit ddocs and feel relief
///because i compress your data so you can enjoy from more space
#[derive(Parser, Debug)]
#[command(version = "0.9")]
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
    if let Some(key) = args.search {
        search(key);
    } else if let Some(key) = args.add {
        add(key);
    } else if let Some(key) = args.cat {
        cat(key);
    } else if let Some(key) = args.edit {
        edit(key);
    } else if let Some(key) = args.delete {
        delete(key);
    }
}
