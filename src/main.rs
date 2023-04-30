#![allow(unused)]

use std::{
    fs::{self, File},
    path::Path,
};

mod commands;
mod compress;
mod editor;
mod serdef;

/*
    now we open json file and put it into hashmap
    for better performance in the future i make a header file
    and split files if they are more than 10mg
    then make the hashmap of array[keys]
*/
fn main() {
    let conf_dir = serdef::path();
    let a = Path::new(&conf_dir);
    if !a.exists() {
        fs::create_dir_all(a.parent().unwrap()).unwrap();
        File::create(&conf_dir);
    };

    commands::run()
}
