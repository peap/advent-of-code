use std::collections::LinkedList;

type Elf = u32;

pub const NUM_ELVES: u32 = 3014603;

pub fn play_white_elephant(num_elves: u32) -> Elf {
    let mut elves: LinkedList<(Elf, u32)> = LinkedList::new();
    for elf in 0..num_elves {
        elves.push_back((elf + 1, 1));
    }
    loop {
        if elves.len() == 1 {
            break;
        }
        let mut thief = elves.pop_front().expect("List should not be empty");
        let victim = elves.pop_front().expect("List should not be empty.");
        thief.1 += victim.1;
        elves.push_back(thief);
    }
    elves.pop_front().expect("List should not be empty.").0
}

fn main() {
    let winner = play_white_elephant(NUM_ELVES);
    println!("Part 1: Elf #{} is the lord of presents!", winner);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let winner = play_white_elephant(5);
        assert_eq!(winner, 3);
    }

    #[test]
    fn test_part_1() {
        let winner = play_white_elephant(NUM_ELVES);
        assert_eq!(winner, 1834903);
    }
}
