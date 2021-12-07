use std::collections::HashMap;

use regex::Regex;

use common::InputReader;

const MIN_LETTER: u8 = b'a';
const NAMED: &str = "northpole-object-storage";

struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

impl Room {
    fn new(name: &str, sector: i32, checksum: &str) -> Room {
        Room {
            name: name.to_string(),
            sector,
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
                char_hash.entry(c).or_insert(1);
            }
        }
        let mut to_sort: Vec<_> = char_hash.iter().collect::<Vec<_>>();
        to_sort.sort_by(|item1, item2| {
            let &(c1, count1) = item1;
            let &(c2, count2) = item2;
            if count1 == count2 {
                c1.cmp(c2)
            } else {
                count2.cmp(count1)
            }
        });
        let expected: String = to_sort.iter().map(|item| *item.0).take(5).collect();
        self.checksum == expected
    }

    pub fn decrypt(&self) -> String {
        let mut rotated: Vec<char> = Vec::new();
        for c in self.name.chars() {
            if c == '-' {
                rotated.push(c);
                continue;
            }
            let to_rotate = (self.sector % 26) as u8;
            let code = (((c as u8).checked_sub(MIN_LETTER))
                .expect("Got a letter < 'a'.")
                .checked_add(to_rotate)
                .expect("Overflow after sector addition!"))
                % 26;
            rotated.push((code + MIN_LETTER) as u8 as char);
        }
        rotated.into_iter().collect()
    }
}

fn load_rooms(lines: Vec<String>) -> Vec<Room> {
    let line_re = Regex::new(r"^([a-z-]+)([0-9]+)\[([a-z]{5})\]$").unwrap();
    let mut rooms = Vec::new();
    for line in lines.iter() {
        let room = match line_re.captures(line) {
            Some(caps) => {
                let name = caps.get(1).unwrap().as_str();
                let sector = caps.get(2).unwrap().as_str().parse().unwrap();
                let checksum = caps.get(3).unwrap().as_str();
                Room::new(name, sector, checksum)
            }
            None => panic!("Unparsable line: {}", line),
        };
        rooms.push(room)
    }
    rooms
}

fn main() {
    let lines = InputReader::new("input.txt").parsed_lines();
    let rooms = load_rooms(lines);
    let valid: Vec<&Room> = rooms.iter().filter(|r| r.is_valid()).collect();
    let sum: i32 = valid.iter().fold(0, |acc, room| acc + room.get_sector());
    println!(
        "Part 1: {} of {} rooms are valid; sector sum is {}",
        valid.len(),
        rooms.len(),
        sum,
    );
    let sector = valid
        .iter()
        .find(|room| room.decrypt().starts_with(NAMED))
        .unwrap()
        .get_sector();
    println!("Part 2: northpole-object-storage is in sector {}", sector);
}
