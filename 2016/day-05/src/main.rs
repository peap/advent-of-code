extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn get_password(id: &str) -> String {
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
            password_chars.push(hashed[2] as char);
            if password_chars.len() >= 8 {
                break;
            }
        }
        hasher.reset();
    }
    password_chars.into_iter().collect()
}

fn main() {
    let id1 = "ffykfhsq";
    let password1 = get_password(id1);
    println!("Part 1: {} --> {}", id1, password1);
}

#[test]
fn test_aoc_example() {
    assert_eq!(get_password("abc"), "18f47a30");
}
