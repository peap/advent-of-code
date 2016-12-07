use std::fs::File;
use std::io::{BufRead, BufReader};

struct IPv7 {
    address: String,
}

impl IPv7 {
    fn new(address: String) -> IPv7 {
        IPv7 { address: address }
    }

    fn supports_tls(&self) -> bool {
        // Would use the regex crate, but version 0.1 doesn't support back
        // references, and I think I need those: "([a-z])([a-z])\2\1". So,
        // we'll scan across the string, instead.
        let mut tls_compliant = false;
        let mut in_brackets = false;
        let mut last_four: [char; 4] = [0 as char; 4];
        for (i, chr) in self.address.chars().enumerate() {
            if i < 3 {
                last_four[i+1] = chr;
                continue;
            };
            // shift
            last_four = [last_four[1], last_four[2], last_four[3], chr];
            if chr == '[' {
                in_brackets = true;
                continue;
            } else if chr == ']' {
                in_brackets = false;
                continue;
            }
            if last_four[0] == last_four[3] &&
                    last_four[1] == last_four[2] &&
                    last_four[0] != last_four[1] {
                if in_brackets {
                    tls_compliant = false;
                    break;
                } else {
                    tls_compliant = true;
                }
            }
        }
        tls_compliant
    }
}

fn load_ipaddrs(filename: &'static str) -> Vec<IPv7> {
    let mut ipaddrs = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(text) => ipaddrs.push(IPv7::new(text)),
            Err(_) => (),
        }
    }
    ipaddrs
}

fn main() {
    let ipaddrs = load_ipaddrs("input.txt");
    let num_tls = ipaddrs.iter().filter(|a| a.supports_tls()).count();
    println!("Part 1: {}/{} IPv7 addrs support TLS", num_tls, ipaddrs.len());
}
