pub const TARGET: u32 = 34_000_000;

const ELF_FACTOR1: usize = 10;
const ELF_FACTOR2: usize = 11;
const HOUSE_LIMIT: usize = 1_000_000;

pub fn find_first_house_receiving(target: u32) -> Option<u32> {
    let mut houses: Vec<u32> = vec![0; HOUSE_LIMIT];
    for elf in 1..HOUSE_LIMIT {
        let mut house = elf;
        while house < HOUSE_LIMIT {
            houses[house] += (elf * ELF_FACTOR1) as u32;
            if houses[house] >= target {
                return Some(house as u32);
            }
            house += elf;
        }
    }
    None
}

pub fn find_first_house_receiving_part_2(target: u32) -> Option<u32> {
    let mut houses: Vec<u32> = vec![0; HOUSE_LIMIT];
    for elf in 1..HOUSE_LIMIT {
        let mut house = elf;
        let mut elf_count = 0;
        while elf_count < 50 {
            houses[house] += (elf * ELF_FACTOR2) as u32;
            if houses[house] >= target {
                return Some(house as u32);
            }
            house += elf;
            elf_count += 1;
            if house >= HOUSE_LIMIT {
                break;
            }
        }
    }
    None
}

fn main() {
    if let Some(num) = find_first_house_receiving(TARGET) {
        println!("Part 1: House {} receives > {} presents", num, TARGET);
    }
    if let Some(num) = find_first_house_receiving_part_2(TARGET) {
        println!("Part 1: House {} receives > {} presents", num, TARGET);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_answer() {
        assert_eq!(find_first_house_receiving(TARGET), Some(786240));
    }

    #[test]
    fn test_part_2_answer() {
        assert_eq!(find_first_house_receiving_part_2(TARGET), Some(831600));
    }
}
