use crate::compress::{compress, decompress};
use crate::serdef::*;
use clap::Parser;
use std::io::{self, stdin, stdout, BufRead, Read, Write};
use termion::{cursor::DetectCursorPos, event::Key, input::TermRead, raw::IntoRawMode};

// for_editing string is for when we want use it for edit option
fn input(for_editing: Option<String>) -> String {
    println!("Write or Edit your doc\npress C-s to save your string\n");
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut input = stdin.keys();
    let mut input_str = String::new();
    if let Some(s) = for_editing {
        input_str.push_str(&s);
        write!(stdout, "{}", input_str).unwrap();
    }
    loop {
        if let Some(Ok(key)) = input.next() {
            match key {
                Key::Ctrl('s') => {
                    break;
                }
                Key::Backspace => {
                    input_str.pop();
                    write!(stdout, "\x08 \x08").unwrap();
                    stdout.flush().unwrap();
                }
                Key::Char('\n') => {
                    input_str.push('\n');
                    write!(stdout, "\r\n").unwrap();
                    // let (x, y) = stdout.cursor_pos().unwrap();
                    // write!(stdout, "{}", termion::cursor::Goto(1, y + 1));
                    stdout.flush().unwrap();
                    // println!("{:?}", stdout.cursor_pos().unwrap());
                }
                Key::Char('\t') => {
                    input_str.push('\t');
                    write!(stdout, "\t").unwrap();
                    stdout.flush().unwrap();
                }
                Key::Char(c) => {
                    input_str.push(c);
                    write!(stdout, "{}", c).unwrap();
                    stdout.flush().unwrap();
                }
                Key::Up => {
                    write!(stdout, "{}", termion::cursor::Up(1)).unwrap();
                    stdout.flush().unwrap();
                }
                Key::Down => {
                    write!(stdout, "{}", termion::cursor::Down(1)).unwrap();
                    stdout.flush().unwrap();
                }
                Key::Left => {
                    write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
                    stdout.flush().unwrap();
                }
                Key::Right => {
                    write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
                    stdout.flush().unwrap();
                }
                _ => {}
            }
        }
        drop(&stdout);
    }

    input_str
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
    let value = input(None);
    let mut map = json_to_hashmap();
    map.insert(key, compress(&value));
    hashmap_to_json(map);
}

pub fn cat(key: String) {
    let mut map = json_to_hashmap();
    if let Some(value) = map.get(&key) {
        println!("{}", decompress(value));
    } else {
        println!("there is no topic with this name");
    }
}

pub fn edit(key: String) {
    let mut map = json_to_hashmap();
    let mut value = map.get(&key).expect("the topic dosent exist");
    let input = input(Some(decompress(value)));
    if let Some(x) = map.get_mut(&key) {
        *x = compress(&input);
    }
    hashmap_to_json(map);
}

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
