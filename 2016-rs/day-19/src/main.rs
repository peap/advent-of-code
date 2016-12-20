use std::collections::{LinkedList, VecDeque};

type Elf = u32;

pub const NUM_ELVES: u32 = 3014603;

pub fn play_white_elephant(num_elves: u32) -> Elf {
    let mut elves: LinkedList<Elf> = LinkedList::new();
    for elf in 0..num_elves {
        elves.push_back(elf + 1);
    }
    while elves.len() > 1 {
        let thief = elves.pop_front().expect("List should not be empty");
        elves.pop_front().expect("List should not be empty.");
        elves.push_back(thief);
    }
    elves.pop_front().expect("List should not be empty.")
}

pub fn play_white_elephant_version_2(num_elves: u32) -> Elf {
    // With thanks to /u/aurele.
    // https://www.reddit.com/r/adventofcode/comments/5j4lp1/2016_day_19_solutions/dbe1o0h/
    let mid = (num_elves + 1) / 2;
    let mut v1: VecDeque<Elf> = (0..mid).collect();
    let mut v2: VecDeque<Elf> = (mid..num_elves).collect();
    loop {
        if v2.len() >= v1.len() {
            v2.pop_front();
            if v2.is_empty() {
                return v1[0] + 1;
            }
        } else {
            v1.pop_back();
        }
        v1.push_back(v2.pop_front().unwrap());
        v2.push_back(v1.pop_front().unwrap());
    }
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

    #[test]
    fn test_more_examples() {
        assert_eq!(play_white_elephant_version_2(7), 5);
        assert_eq!(play_white_elephant_version_2(9), 9);
        assert_eq!(play_white_elephant_version_2(16), 7);
    }

    #[test]
    fn test_part_2() {
        let winner = play_white_elephant_version_2(NUM_ELVES);
        assert_eq!(winner, 1420280);
    }

}
