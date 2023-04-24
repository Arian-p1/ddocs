use crate::{compress::*, serdef::*};

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
    let mut hashmap = json_to_hashmap();
    hashmap.insert(key, compresse(&value));
    hashmap_to_json(hashmap).unwrap();
}

pub fn cat(key: String) {
    let mut map = json_to_hashmap();
    if let Some(value) = map.get(&key) {
        println!("{}", decompress(value.to_owned()));
    } else {
        println!("there is no topic with this name");
    }
}

// soon
pub fn edit(key: String) {}

pub fn delete(key: String) {
    let mut map = json_to_hashmap();
    map.remove(&key);
    hashmap_to_json(map);
    println!("the topic is deleted");
}
