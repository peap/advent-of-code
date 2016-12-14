extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

pub const MY_SALT: &'static str = "ngcjuoqr";
const NEXTN: u64 = 1000;

pub fn find_consecutive<'a>(n: u32, string: &'a str) -> Option<char> {
    let mut last = 0 as char;
    let mut count: u32 = 0;
    for c in string.chars() {
        if c == last {
            count += 1;
        } else {
            last = c;
            count = 1;
        }
        if count == n {
            return Some(c);
        }
    }
    None
}

pub fn get_index_that_produces_n_keys<'a>(n: usize, salt: &'a str) -> u64 {
    let salt_bytes = salt.as_bytes();
    let mut potentials_keys: Vec<(String, char, u64)> = Vec::new();
    let mut completed_keys: Vec<usize> = Vec::new();
    let mut keys: Vec<String> = Vec::new();
    let mut hasher = Md5::new();
    let mut output = String::new();
    for i in 0..std::u64::MAX {
        hasher.input(salt_bytes);
        hasher.input(i.to_string().as_bytes());
        output = hasher.result_str();
        // look for 5 in a row
        if let Some(char5) = find_consecutive(5, &output) {
            // compare previous 3-in-a-row finds and see if this 5-er makes
            // them a real key
            for (j, &(ref key, ref char3, ref idx)) in
                    potentials_keys.iter().enumerate() {
                if *char3 == char5 && i - idx <= NEXTN {
                    keys.push(key.clone());
                    completed_keys.push(j);
                }
                if i - idx > NEXTN {
                    // this match is too old now
                    completed_keys.push(j);
                }
                if keys.len() == n {
                    // finished!
                    return *idx;
                }
            }
            // sort the indices that we need to remove from potentials_keys so
            // that we process them in order and can safely subtract off the
            // number of keys we've taken out so far (k)
            completed_keys.sort();
            for (k, idx) in completed_keys.iter().enumerate() {
                potentials_keys.remove(*idx - k);
            }
            completed_keys.clear();
        }
        // look for new triples
        if let Some(char3) = find_consecutive(3, &output) {
            potentials_keys.push((output, char3, i));
        }
        hasher.reset();
    }
    u64::max_value()
}

fn main() {
    let index = get_index_that_produces_n_keys(64, MY_SALT);
    println!("Part 1: index #{} produces the 64th key", index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finding_consecutive_letters1() {
        let c3 = find_consecutive(3, "abcabcccddd");
        assert_eq!(c3, Some('c'));
        let e4 = find_consecutive(4, "eeeeabcffffabcccddd");
        assert_eq!(e4, Some('e'));
        let none = find_consecutive(7, "eeeeabcffffabcccddd");
        assert_eq!(none, None);
    }

    #[test]
    fn test_example() {
        let index = get_index_that_produces_n_keys(64, "abc");
        assert_eq!(index, 22728);
    }

    #[test]
    fn test_part_1() {
        let index = get_index_that_produces_n_keys(64, MY_SALT);
        assert_eq!(index, 18626);
    }
}
