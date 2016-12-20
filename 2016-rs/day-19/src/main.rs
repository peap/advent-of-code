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

pub fn play_white_elephant_version_2(num_elves: u32) -> Elf {
    let mut elves: LinkedList<(Elf, u32)> = LinkedList::new();
    for elf in 0..num_elves {
        elves.push_back((elf + 1, 1));
    }
    loop {
        if elves.len() == 1 {
            break;
        }
        println!("\rElves: {}     ", elves.len());
        let mut thief = elves.pop_front().expect("List should not be empty");
        let idx = elves.len() / 2 - if elves.len() % 2 == 0 { 1 } else { 0 };
        let mut back_half = elves.split_off(idx);
        let victim = back_half.pop_front().expect("List should not be empty.");
        elves.append(&mut back_half);
        thief.1 += victim.1;
        elves.push_back(thief);
    }
    elves.pop_front().expect("List should not be empty.").0
}

fn main() {
    let winner_1 = play_white_elephant(NUM_ELVES);
    println!("Part 1: Elf #{} is the lord of presents!", winner_1);
    let winner_2 = play_white_elephant_version_2(NUM_ELVES);
    println!("Part 2: Elf #{} is the lord of presents!", winner_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let winner = play_white_elephant(5);
        assert_eq!(winner, 3);
    }

    #[test]
    fn test_part_1() {
        let winner = play_white_elephant(NUM_ELVES);
        assert_eq!(winner, 1834903);
    }

    #[test]
    fn test_example_2() {
        let winner = play_white_elephant_version_2(5);
        assert_eq!(winner, 2);
    }
}
