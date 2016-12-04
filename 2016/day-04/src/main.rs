extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;


struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

impl Room {
    fn new(name: &str, sector: i32, checksum: &str) -> Room {
        Room {
            name: name.to_string(),
            sector: sector,
            checksum: checksum.to_string(),
        }
    }

    pub fn get_sector(&self) -> i32 {
        self.sector
    }

    fn is_valid(&self) -> bool {
        let mut char_hash: HashMap<char, i32> = HashMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            } else {
                if let Some(count) = char_hash.get_mut(&c) {
                    *count += 1;
                }
                if !char_hash.contains_key(&c) {
                    char_hash.insert(c, 1);
                }
            }
        }
        let mut to_sort: Vec<_> = char_hash.iter()
            .collect::<Vec<_>>();
        to_sort.sort_by(|item1, item2| {
                let &(c1, count1) = item1;
                let &(c2, count2) = item2;
                if count1 == count2 {
                    c1.cmp(c2)
                } else {
                    count2.cmp(count1)
                }
            });
        let expected: String = to_sort.iter()
                                   .map(|item| item.0.clone())
                                   .take(5)
                                   .collect();
        self.checksum == expected
    }
}

fn load_rooms(filename: &'static str) -> Vec<Room> {
    let line_re = Regex::new(r"^([a-z-]+)([0-9]+)\[([a-z]{5})\]$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut rooms = Vec::new();
    for line in reader.lines() {
        let text = line.expect("Couldn't read a line.");
        let room = match line_re.captures(&text) {
            Some(caps) => {
                let name = caps.at(1).unwrap();
                let sector = caps.at(2).unwrap().parse().unwrap();
                let checksum = caps.at(3).unwrap();
                Room::new(name, sector, checksum)
            }
            None => panic!("Unparsable line: {}", text)
        };
        rooms.push(room)
    }
    rooms
}

fn main() {
    let rooms = load_rooms("input.txt");
    let valid: Vec<&Room> = rooms.iter().filter(|r| r.is_valid()).collect();
    let sum: i32 = valid.iter().fold(0, |acc, room| acc + room.get_sector());
    println!(
        "Part 1: {} of {} rooms are valid; sector sum is {}",
        valid.len(), rooms.len(), sum,
    );
}
