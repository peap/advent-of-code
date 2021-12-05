#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
    static ref BOT_REGEX: regex::Regex = {
        regex::Regex::new(
            "^bot ([0-9]+) gives low to (output|bot) ([0-9]+) \
                          and high to (output|bot) ([0-9]+)$",
        )
        .expect("Invalid bot regex.")
    };
    static ref VALUE_REGEX: regex::Regex = {
        regex::Regex::new("^value ([0-9]+) goes to bot ([0-9]+)$").expect("Invalid value regex.")
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Chip {
    Value(u32),
    Empty,
}

impl Ord for Chip {
    fn cmp(&self, other: &Chip) -> Ordering {
        use Chip::*;
        match *self {
            Empty => match *other {
                Empty => Ordering::Equal,
                Value(_) => Ordering::Less,
            },
            Value(n) => match *other {
                Empty => Ordering::Greater,
                Value(m) => n.cmp(&m),
            },
        }
    }
}

impl PartialOrd for Chip {
    fn partial_cmp(&self, other: &Chip) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Recipient {
    Bot(u32),
    Output(u32),
}

impl Recipient {
    fn from_text<'a>(kind: &'a str, id: &'a str) -> Recipient {
        let id: u32 = id
            .parse()
            .expect("Recipient IDs must be positive integers.");
        match kind {
            "bot" => Recipient::Bot(id),
            "output" => Recipient::Output(id),
            _ => panic!("Invalid recipient: {} {}", kind, id),
        }
    }
}

pub struct Bot {
    id: u32,
    chips: [Chip; 2],
    give_low_to: Recipient,
    give_high_to: Recipient,
}

impl Bot {
    fn from_text(text: String) -> Bot {
        let caps = BOT_REGEX.captures(&text).expect("No bot regex captures.");
        let bot_id: u32 = caps
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .expect("Non-numeric bot ID.");
        let low_recip_type = caps.get(2).unwrap().as_str();
        let low_recip_id = caps.get(3).unwrap().as_str();
        let low_recip = Recipient::from_text(low_recip_type, low_recip_id);
        let high_recip_type = caps.get(4).unwrap().as_str();
        let high_recip_id = caps.get(5).unwrap().as_str();
        let high_recip = Recipient::from_text(high_recip_type, high_recip_id);
        Bot {
            id: bot_id,
            chips: [Chip::Empty, Chip::Empty],
            give_low_to: low_recip,
            give_high_to: high_recip,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn give_low_to(&self) -> Recipient {
        self.give_low_to.clone()
    }

    pub fn give_high_to(&self) -> Recipient {
        self.give_high_to.clone()
    }

    pub fn hands_full(&self) -> bool {
        self.chips[0] != Chip::Empty && self.chips[1] != Chip::Empty
    }

    pub fn receive_chip(&mut self, chip: Chip) {
        if self.chips[0] == Chip::Empty {
            self.chips[0] = chip;
        } else if self.chips[1] == Chip::Empty {
            self.chips[1] = chip;
            self.chips.sort();
        } else {
            panic!("Bot {} already has two chips!", self.id());
        }
    }

    pub fn release_low_chip(&mut self) -> Chip {
        let chip = self.chips[0].clone();
        self.chips[0] = Chip::Empty;
        chip
    }

    pub fn release_high_chip(&mut self) -> Chip {
        let chip = self.chips[1].clone();
        self.chips[1] = Chip::Empty;
        chip
    }
}

pub fn load_bots<'a>(filename: &'a str) -> HashMap<u32, Bot> {
    let mut bots = HashMap::new();
    let mut starting_values: Vec<String> = Vec::new(); // process after bots
    let f = File::open(filename).expect("Could not find input file.");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let text = line.expect("Error reading input file.");
        if BOT_REGEX.is_match(&text) {
            let bot = Bot::from_text(text);
            bots.insert(bot.id(), bot);
        } else if VALUE_REGEX.is_match(&text) {
            // file this away until the end, after we know about all the bots
            starting_values.push(text)
        } else {
            println!("Unparsable text: {}", text);
        }
    }
    for text in starting_values {
        let caps = VALUE_REGEX
            .captures(&text)
            .expect("Found match to value regex, but no captures.");
        let value: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let bot_id: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let bot = bots
            .get_mut(&bot_id)
            .expect("Trying to give value to nonexistant bot.");
        bot.receive_chip(Chip::Value(value));
    }
    bots
}

fn get_active_bot_ids(bots: &HashMap<u32, Bot>) -> Option<Vec<u32>> {
    let mut active_bots = Vec::new();
    for (bot_id, bot) in bots.iter() {
        if bot.hands_full() {
            active_bots.push(*bot_id);
        }
    }
    match active_bots.len() {
        0 => None,
        _ => Some(active_bots),
    }
}

pub fn pass_chips_until<F>(bots: &mut HashMap<u32, Bot>, predicate: F) -> Option<u32>
where
    F: Fn(&Bot) -> bool,
{
    let mut active_bot_ids = match get_active_bot_ids(&bots) {
        Some(ids) => ids,
        None => panic!("Expected one bot to be active at the start!"),
    };
    loop {
        for id in active_bot_ids {
            let low_chip: Chip;
            let high_chip: Chip;
            let low_recip: Recipient;
            let high_recip: Recipient;
            {
                // Introduce scope to let this mutable borrow expire.
                let bot = bots.get_mut(&id).unwrap();
                if predicate(bot) {
                    return Some(id);
                }
                low_chip = bot.release_low_chip();
                low_recip = bot.give_low_to();
                high_chip = bot.release_high_chip();
                high_recip = bot.give_high_to();
            }
            match low_recip {
                Recipient::Bot(id) => {
                    let receiver = bots.get_mut(&id).unwrap();
                    receiver.receive_chip(low_chip);
                }
                Recipient::Output(n) => {
                    if n <= 2 {
                        println!("Output {}: Chip {:?}", n, low_chip);
                    }
                }
            };
            match high_recip {
                Recipient::Bot(id) => {
                    let receiver = bots.get_mut(&id).unwrap();
                    receiver.receive_chip(high_chip);
                }
                Recipient::Output(n) => {
                    if n <= 2 {
                        println!("Output {}: Chip {:?}", n, high_chip);
                    }
                }
            };
        }
        active_bot_ids = match get_active_bot_ids(&bots) {
            Some(ids) => ids,
            None => break,
        }
    }
    None
}

fn main() {
    let mut bots = load_bots("input.txt");
    let part1_pred = |b: &Bot| b.chips[0] == Chip::Value(17) && b.chips[1] == Chip::Value(61);
    if let Some(part1_bot_id) = pass_chips_until(&mut bots, part1_pred) {
        println!("Part 1: Bot {} compares the 17 and 61 chips.", part1_bot_id);
    } else {
        println!("Part 1: couldn't identify the bot :/");
    }
    // keep going until the end to see where all the chips go
    pass_chips_until(&mut bots, |_| false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_answer() {
        let mut bots = load_bots("input.txt");
        let part1_pred = |b: &Bot| b.chips[0] == Chip::Value(17) && b.chips[1] == Chip::Value(61);
        let result = pass_chips_until(&mut bots, part1_pred);
        assert_eq!(result, Some(98));
    }
}
