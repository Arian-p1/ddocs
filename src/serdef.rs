use crate::compress::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Result, Value};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufWriter},
    path::Path,
};

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    key: String,
    value: String,
}

pub fn path() -> String {
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
    let file = File::options().read(true).write(true).open(&conf_dir);
    file
}

/// it will also write the json into config file
pub fn hashmap_to_json(map: HashMap<String, String>) -> io::Result<()> {
    let writer = BufWriter::new(file().unwrap());

    serde_json::to_writer_pretty(
        writer,
        &Value::Array(
            map.into_iter()
                .map(|(key, value)| json!({"key": key, "value": value}))
                .collect(),
        ),
    )?;

    Ok(())
}

pub fn json_to_hashmap() -> HashMap<String, String> {
    let st = fs::read_to_string(path().as_str()).unwrap();
    let data = serde_json::from_str::<Vec<Data>>(&st);
    let mut map = HashMap::new();

    match data {
        Ok(a) => {
            for item in a {
                map.insert(item.key, item.value);
            }
        }
        Err(_) => {}
    }

    map
}
