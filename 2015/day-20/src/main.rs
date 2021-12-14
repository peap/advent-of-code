use common::{default_puzzle, Answer, InputReader, Puzzle};

const ELF_FACTOR1: usize = 10;
const ELF_FACTOR2: usize = 11;
const HOUSE_LIMIT: usize = 1_000_000;

pub fn find_first_house_receiving(target: u64) -> Option<u64> {
    let mut houses: Vec<u64> = vec![0; HOUSE_LIMIT];
    for elf in 1..HOUSE_LIMIT {
        let mut house = elf;
        while house < HOUSE_LIMIT {
            houses[house] += (elf * ELF_FACTOR1) as u64;
            if houses[house] >= target {
                return Some(house as u64);
            }
            house += elf;
        }
    }
    None
}

pub fn find_first_house_receiving_part_2(target: u64) -> Option<u64> {
    let mut houses: Vec<u64> = vec![0; HOUSE_LIMIT];
    for elf in 1..HOUSE_LIMIT {
        let mut house = elf;
        let mut elf_count = 0;
        while elf_count < 50 {
            houses[house] += (elf * ELF_FACTOR2) as u64;
            if houses[house] >= target {
                return Some(house as u64);
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

fn part1(reader: &InputReader) -> Answer {
    let target: u64 = reader.string_line().parse().unwrap();
    find_first_house_receiving(target).unwrap()
}

fn part2(reader: &InputReader) -> Answer {
    let target: u64 = reader.string_line().parse().unwrap();
    find_first_house_receiving_part_2(target).unwrap()
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Infinite Elves and Infinite Houses");
    puzzle.set_part1(part1, "lowest house number");
    puzzle.set_part2(part2, "lowest house number");
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(786240);
    }

    #[test]
    fn test_part_2_answer() {
        get_puzzle().test_part2(831600);
    }
}
