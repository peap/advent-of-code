extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Chip {
    Value(u32),
    Empty,
}

#[derive(Debug, PartialEq)]
enum Recipient {
    Bot(u32),
    Output(u32),
}

impl Recipient {
    fn from_text<'a>(kind: &'a str, id: &'a str) -> Recipient {
        let id: u32 = id.parse().expect("Recipient IDs must be positive integers.");
        match kind {
            "bot" => Recipient::Bot(id),
            "output" => Recipient::Output(id),
            _ => panic!("Invalid recipient: {} {}", kind, id),
        }
    }
}

struct Bot {
    id: u32,
    chips: [Chip; 2],
    give_low_to: Recipient,
    give_high_to: Recipient,
}

impl Bot {
    fn from_text(text: String) -> Bot {
        let bot_re = get_bot_regex();
        let caps = bot_re.captures(&text).expect("No bot regex captures.");
        let bot_id: u32 = caps.at(1).unwrap().parse().expect("Non-numeric bot ID.");
        let low_recip_type = caps.at(2).unwrap();
        let low_recip_id = caps.at(3).unwrap();
        let low_recip = Recipient::from_text(low_recip_type, low_recip_id);
        let high_recip_type = caps.at(4).unwrap();
        let high_recip_id = caps.at(5).unwrap();
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

    pub fn hands_full(&self) -> bool {
        self.chips[0] != Chip::Empty && self.chips[1] != Chip::Empty
    }

    pub fn take_chip(&mut self, chip: Chip) {
        if self.chips[0] == Chip::Empty {
            self.chips[0] = chip;
        } else if self.chips[1] == Chip::Empty {
            self.chips[1] = chip;
        } else {
            panic!("Bot {} already has two chips!", self.id());
        }
    }
}

fn get_bot_regex() -> regex::Regex {
    regex::Regex::new("^bot ([0-9]+) gives low to (output|bot) ([0-9]+) \
                      and high to (output|bot) ([0-9]+)$")
        .expect("Invalid bot regex.")
}

fn get_value_regex() -> regex::Regex {
    regex::Regex::new("^value ([0-9]+) goes to bot ([0-9]+)$")
        .expect("Invalid value regex.")
}

fn load_bots<'a>(filename: &'a str) -> HashMap<u32, Bot> {
    let mut bots = HashMap::new();
    let mut starting_values: Vec<String> = Vec::new(); // process after bots
    let bot_re = get_bot_regex();
    let value_re = get_value_regex();
    let f = File::open(filename).expect("Could not find input file.");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let text = line.expect("Error reading input file.");
        if bot_re.is_match(&text) {
            let bot = Bot::from_text(text);
            bots.insert(bot.id(), bot);
        } else if value_re.is_match(&text) {
            // file this away until the end, after we know about all the bots
            starting_values.push(text)
        } else {
            println!("Unparsable text: {}", text);
        }
    }
    for text in starting_values {
        let caps = value_re.captures(&text)
            .expect("Found match to value regex, but no captures.");
        let value: u32 = caps.at(1).unwrap().parse().unwrap();
        let bot_id: u32 = caps.at(2).unwrap().parse().unwrap();
        let mut bot = bots.get_mut(&bot_id)
            .expect("Trying to give value to nonexistant bot.");
        bot.take_chip(Chip::Value(value));
    }
    bots
}

fn main() {
    let bots = load_bots("input.txt");
    for (bot_id, bot) in bots.iter() {
        if bot.hands_full() {
            println!("Bot {} has its hands full.", bot.id());
        }
    }
}
