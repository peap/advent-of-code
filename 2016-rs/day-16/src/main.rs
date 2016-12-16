pub const INITIAL_STATE: &'static str = "11101000110010100";
pub const DISK_LEN_1: usize = 272;
pub const DISK_LEN_2: usize = 35651584;

pub fn expand<'a>(state: &'a str) -> String {
    let mut expanded = String::new();
    expanded.push_str(state);
    expanded.push('0');
    for c in state.chars().rev() {
        match c {
            '0' => expanded.push('1'),
            '1' => expanded.push('0'),
            _ => ()
        }
    }
    expanded
}

fn get_checksum(disk: String) -> String {
    let vec: Vec<char> = disk.chars().collect();
    let mut checksum = String::new();
    let mut i = 0;
    while i < vec.len() - 1 {
        checksum.push(if vec[i] == vec[i + 1] { '1' } else { '0' });
        i += 2;
    }
    checksum
}

pub fn fill_and_checksum<'a>(state: &'a str, disk_length: usize) -> String {
    let mut disk = String::new();
    let mut expanded = expand(state);
    while expanded.len() < disk_length {
        expanded = expand(&expanded);
    }
    for c in expanded.chars() {
        disk.push(c);
        if disk.len() == disk_length {
            break;
        }
    }
    let mut checksum = get_checksum(disk);
    while checksum.len() % 2 == 0 {
        checksum = get_checksum(checksum);
    }
    checksum
}

fn main() {
    let checksum = fill_and_checksum(INITIAL_STATE, DISK_LEN_1);
    println!("Part 1: checksum is {}", checksum);
    let checksum = fill_and_checksum(INITIAL_STATE, DISK_LEN_2);
    println!("Part 2: checksum is {}", checksum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansion() {
        assert_eq!(expand("1"), "100");
        assert_eq!(expand("0"), "001");
        assert_eq!(expand("11111"), "11111000000");
        assert_eq!(expand("111100001010"), "1111000010100101011110000");
    }

    #[test]
    fn test_example_1() {
        let checksum = fill_and_checksum("110010110100", 12);
        assert_eq!(checksum, "100");
    }

    #[test]
    fn test_example_2() {
        let checksum = fill_and_checksum("10000", 20);
        assert_eq!(checksum, "01100");
    }

    #[test]
    fn test_part_1() {
        let checksum = fill_and_checksum(INITIAL_STATE, DISK_LEN_1);
        assert_eq!(checksum, "10100101010101101");
    }

    #[test]
    fn test_part_2() {
        let checksum = fill_and_checksum(INITIAL_STATE, DISK_LEN_2);
        assert_eq!(checksum, "01100001101101001");
    }

}
