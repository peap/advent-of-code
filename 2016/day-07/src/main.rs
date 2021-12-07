use std::str::FromStr;

use common::{BadInput, InputReader};

struct IPv7 {
    address: String,
}

impl FromStr for IPv7 {
    type Err = BadInput;

    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        Ok(IPv7 {
            address: addr.to_string(),
        })
    }
}

impl IPv7 {
    fn supports_tls(&self) -> bool {
        // Would use the regex crate, but version 0.1 doesn't support back
        // references, and I think I need those: "([a-z])([a-z])\2\1". So,
        // we'll scan across the string, instead.
        let mut tls_compliant = false;
        let mut in_brackets = false;
        let mut last_four: [char; 4] = [0 as char; 4];
        for (i, chr) in self.address.chars().enumerate() {
            if i < 3 {
                last_four[i + 1] = chr;
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
            if last_four[0] == last_four[3]
                && last_four[1] == last_four[2]
                && last_four[0] != last_four[1]
            {
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

    fn supports_ssl(&self) -> bool {
        // Would use the regex crate, but version 0.1 doesn't support back
        // references, and I think I need those: "([a-z])([a-z])\1.*\[[^\]]*\2\1\2".
        // So, we'll scan across the parts of the string and look for
        // candidates, instead.
        // Split address into alternating parts: supernet, [hypernet],
        // supernet, [hypernet], etc. There can be multiple hypernets, but none
        // of the addresses *start* with a hypernet, so we can assume that even
        // indices of the split are supernets and odd are hypernets.
        let parts: Vec<&str> = self.address.split(|c: char| c == '[' || c == ']').collect();
        let mut supernets: Vec<&str> = Vec::new();
        let mut hypernets: Vec<&str> = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            if i % 2 == 0 {
                supernets.push(part);
            } else {
                hypernets.push(part);
            }
        }
        // Look for BAB in the hypernets, then look for the corresponding ABA
        // in the supernets.
        let mut bab_candidates: Vec<String> = Vec::new();
        for hypernet in hypernets.iter() {
            let babs = self.get_babs(hypernet);
            for bab in babs {
                bab_candidates.push(bab);
            }
        }
        if bab_candidates.is_empty() {
            false
        } else {
            // Look for corresponding ABA in supernets.
            let mut found_match = false;
            for bab in bab_candidates {
                let bytes = bab.as_bytes();
                let aba: String = [bytes[1], bytes[0], bytes[1]]
                    .iter()
                    .map(|c| *c as char)
                    .collect();
                for supernet in supernets.iter() {
                    if supernet.contains(&aba) {
                        found_match = true;
                        break;
                    }
                }
                if found_match {
                    break;
                }
            }
            found_match
        }
    }

    fn get_babs(&self, net: &str) -> Vec<String> {
        let mut babs = Vec::new();
        let mut last3: [char; 3] = [0 as char; 3];
        for (i, chr) in net.chars().enumerate() {
            if i < 2 {
                last3[i + 1] = chr;
                continue;
            }
            last3 = [last3[1], last3[2], chr];
            if last3[0] == last3[2] && last3[0] != last3[1] {
                babs.push(last3.iter().copied().collect())
            }
        }
        babs
    }
}

fn main() {
    let ipaddrs = InputReader::new("input.txt").parsed_lines::<IPv7>();
    let num_tls = ipaddrs.iter().filter(|a| a.supports_tls()).count();
    let num_ssl = ipaddrs.iter().filter(|a| a.supports_ssl()).count();
    println!(
        "Part 1: {}/{} IPv7 addrs support TLS",
        num_tls,
        ipaddrs.len()
    );
    println!(
        "Part 2: {}/{} IPv7 addrs support SSL",
        num_ssl,
        ipaddrs.len()
    );
}
