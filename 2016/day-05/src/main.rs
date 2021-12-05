extern crate crypto;

use std::collections::HashSet;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn get_password_1(id: &str, length: usize) -> String {
    let key_base = id.as_bytes();
    let mut password_chars: Vec<char> = Vec::new();
    let mut hasher = Md5::new();
    let mut hashed: [u8; 16] = [0; 16];
    for i in 0..std::u64::MAX {
        hasher.input(key_base);
        hasher.input(i.to_string().as_bytes());
        hasher.result(&mut hashed);
        // Check sum of first five digits in hex representation. Thanks, gkbrk!
        // See: https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
        if hashed[0] as i32 + hashed[1] as i32 + (hashed[2] >> 4) as i32 == 0 {
            password_chars.push(hasher.result_str().chars().nth(5).unwrap());
            if password_chars.len() >= length {
                break;
            }
        }
        hasher.reset();
    }
    password_chars.into_iter().collect()
}

fn get_password_2(id: &str, length: usize) -> String {
    let key_base = id.as_bytes();
    let mut password_chars: Vec<char> = vec![0 as char; length];
    let mut indices: HashSet<usize> = HashSet::new();
    let mut hasher = Md5::new();
    let mut hashed: [u8; 16] = [0; 16];
    for i in 0..std::u64::MAX {
        hasher.input(key_base);
        hasher.input(i.to_string().as_bytes());
        hasher.result(&mut hashed);
        if hashed[0] as i32 + hashed[1] as i32 + (hashed[2] >> 4) as i32 == 0 {
            let hashed_str = hasher.result_str();
            let mut hex_chars = hashed_str.chars(); // .nth() requires mut
            let index = hex_chars.nth(5).unwrap().to_string().parse::<usize>();
            match index {
                Ok(idx) => {
                    if idx < length && !indices.contains(&idx) {
                        // first 6 chars already consumed
                        let chr = hex_chars.nth(0).unwrap();
                        password_chars[idx] = chr;
                        indices.insert(idx);
                    }
                }
                _ => (),
            }
            if indices.len() == length {
                break;
            }
        }
        hasher.reset();
    }
    password_chars.into_iter().collect()
}

fn main() {
    let id = "ffykfhsq";
    let password1 = get_password_1(id, 8);
    println!("Part 1: {} --> {}", id, password1);
    let password2 = get_password_2(id, 8);
    println!("Part 2: {} --> {}", id, password2);
}

#[test]
fn test_aoc_example_1() {
    assert_eq!(get_password_1("abc", 8), "18f47a30");
}

#[test]
#[ignore] // 65.0s
fn test_aoc_example_2() {
    assert_eq!(get_password_2("abc", 8), "05ace8e3");
}
