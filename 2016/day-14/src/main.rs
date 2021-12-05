extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

pub const MY_SALT: &'static str = "ngcjuoqr";
const MAX_AGE: u64 = 1000;

pub fn find_consecutive<'a>(n: u32, string: &'a str) -> Option<char> {
    let mut last = 0 as char;
    let mut count: u32 = 0;
    for c in string.chars() {
        count = if c == last { count + 1 } else { 1 };
        if count == n {
            // Only care about first set of n consecutive letters.
            return Some(c);
        }
        last = c;
    }
    None
}

pub fn get_index_that_produces_n_keys<'a>(n: usize, salt: &'a str, extra: u32) -> u64 {
    let mut potentials_keys: Vec<(String, char, u64)> = Vec::new();
    let mut no_longer_potential: Vec<usize> = Vec::new();
    let mut keys: Vec<u64> = Vec::new();
    let mut hasher = Md5::new();
    let mut output: String;
    for i in 0..std::u64::MAX {
        hasher.reset();
        hasher.input_str(salt);
        hasher.input_str(&i.to_string());
        output = hasher.result_str();
        for _ in 0..extra {
            hasher.reset();
            hasher.input_str(&output);
            output = hasher.result_str();
        }
        // look for 5 in a row
        if let Some(char5) = find_consecutive(5, &output) {
            // compare previous 3-in-a-row finds and see if this 5-er makes
            // them a real key
            for (j, &(_, ref char3, ref idx)) in potentials_keys.iter().enumerate() {
                let age = i - idx;
                if *char3 == char5 && age <= MAX_AGE {
                    no_longer_potential.push(j);
                    keys.push(*idx);
                    keys.sort();
                }
                if age > MAX_AGE {
                    // this match is too old now
                    no_longer_potential.push(j);
                }
                if keys.len() >= n {
                    if let Some(last) = keys.last() {
                        // make sure we've gone at least MAX_AGE past the last
                        // triple, looking for quintuples
                        if last + MAX_AGE < i {
                            return keys[n - 1];
                        }
                    }
                }
            }
            // sort the indices that we need to remove from potentials_keys so
            // that we process them in order and can safely subtract off the
            // number of keys we've taken out so far (k)
            no_longer_potential.sort();
            for (k, idx) in no_longer_potential.iter().enumerate() {
                potentials_keys.remove(*idx - k);
            }
            no_longer_potential.clear();
        }
        // look for new triples
        if let Some(char3) = find_consecutive(3, &output) {
            potentials_keys.push((output, char3, i));
        }
    }
    u64::max_value()
}

fn main() {
    let index = get_index_that_produces_n_keys(64, MY_SALT, 0);
    println!("Part 1: index #{} produces the 64th key", index);
    let index2 = get_index_that_produces_n_keys(64, MY_SALT, 2016);
    println!("Part 2: index #{} produces the 64th key", index2);
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
    fn test_example_1() {
        let index = get_index_that_produces_n_keys(64, "abc", 0);
        assert_eq!(index, 22728);
    }

    #[test]
    fn test_part_1() {
        let index = get_index_that_produces_n_keys(64, MY_SALT, 0);
        assert_eq!(index, 18626);
    }

    #[test]
    #[ignore] // 426s
    fn test_example_2() {
        let index = get_index_that_produces_n_keys(64, "abc", 2016);
        assert_eq!(index, 22551);
    }

    #[test]
    #[ignore] // 426s
    fn test_part_2() {
        let index = get_index_that_produces_n_keys(64, MY_SALT, 2016);
        assert_eq!(index, 20092);
    }
}
