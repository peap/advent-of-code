pub const PART_1_NUM: u32 = 34_000_000;

const ELF_FACTOR: usize = 10;
const LIMIT: usize = 1_000_000;

pub fn find_first_house_receiving(n_presents: u32) -> Option<u32> {
    let mut houses: Vec<u32> = vec![0; LIMIT];
    for elf in 1..LIMIT {
        let mut house = elf;
        while house < LIMIT {
            houses[house] += (elf * ELF_FACTOR) as u32;
            if houses[house] >= n_presents {
                return Some(house as u32);
            }
            house += elf;
        }
    }
    None
}

fn main() {
    if let Some(num) = find_first_house_receiving(PART_1_NUM) {
        println!("Part 1: House {} receives > {} presents", num, PART_1_NUM);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_answer() {
        assert_eq!(find_first_house_receiving(PART_1_NUM), Some(786240));
    }

}
