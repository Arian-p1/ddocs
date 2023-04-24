use crate::compress::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    path::Path,
};

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    key: String,
    value: String,
}

fn path() -> String {
    let home = match std::env::home_dir() {
        Some(a) => a.to_owned(),
        _ => panic!("not fount path"),
    };

    let conf_dir = home.to_str().unwrap().to_owned() + "/.config/ddocs/ddocs.json";
    conf_dir
}

// make path if not exist then open file
fn file() -> std::result::Result<File, io::Error> {
    let conf_dir = path();
    let a = Path::new(&conf_dir);
    let file = if !a.exists() {
        fs::create_dir_all(a.parent().unwrap()).unwrap();
        File::create(&conf_dir)?;
        File::options().read(true).write(true).open(&conf_dir)?
    } else {
        File::options().read(true).write(true).open(&conf_dir)?
    };
    Ok(file)
}

// haminja file ro write kon bere
pub fn hashmap_to_json(map: HashMap<String, String>) -> Result<()> {
    let data: Vec<Data> = map
        .into_iter()
        .map(|(k, v)| Data { key: k, value: v })
        .collect();
    let json = serde_json::to_string(&data)?;

    Ok(())
}

pub fn json_to_hashmap() -> HashMap<String, String> {
    let st = fs::read_to_string(path().as_str()).unwrap();
    let data: Vec<Data> = serde_json::from_str(&st).unwrap();

    let mut map = HashMap::new();
    for item in data {
        map.insert(item.key, item.value);
    }
    map
}
