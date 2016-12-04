use std::fs::File;
use std::io::{BufRead, BufReader};

struct Room { }

impl Room {
    fn from_line(line: String) -> Room {
        Room { }
    }
}

fn load_rooms(filename: &'static str) -> Vec<Room> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut rooms = Vec::new();
    for line in reader.lines() {
        rooms.push(Room::from_line(line.expect("Couldn't read a line.")))
    }
    rooms
}

fn main() {
    let rooms = load_rooms("input.txt");
    println!("Loaded {} rooms.", rooms.len());
}
