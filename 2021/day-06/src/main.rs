use common::InputReader;

const CYCLE: usize = 7;
const NEW_FISH_BUFFER: usize = 2;

fn simulate_lanternfish(fish: Vec<u8>, days: u64) -> u64 {
    let ages = CYCLE + NEW_FISH_BUFFER;
    let mut fish_timers = vec![0; ages];
    for f in fish.iter() {
        fish_timers[*f as usize] += 1;
    }
    for _ in 0..days {
        let new_fish = fish_timers[0];
        for d in 0..ages {
            if d < ages - 1 {
                fish_timers[d] = fish_timers[d + 1]
            } else {
                fish_timers[d] = new_fish
            }
        }
        fish_timers[CYCLE - 1] += new_fish;
    }
    fish_timers.iter().sum()
}

fn part1() -> u64 {
    let num_strs = InputReader::new("input.txt").csv_line();
    let fish: Vec<u8> = num_strs.iter().map(|s| s.parse().unwrap()).collect();
    simulate_lanternfish(fish, 80)
}

fn part2() -> u64 {
    let num_strs = InputReader::new("input.txt").csv_line();
    let fish: Vec<u8> = num_strs.iter().map(|s| s.parse().unwrap()).collect();
    simulate_lanternfish(fish, 256)
}

fn main() {
    println!("Part 1: num fish after 80 cycles:  {}", part1());
    println!("Part 2: num fish after 256 cycles: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let fish: Vec<u8> = vec![3, 4, 3, 1, 2];
        assert_eq!(simulate_lanternfish(fish.clone(), 80), 5934);
        assert_eq!(simulate_lanternfish(fish, 256), 26984457539);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 366057);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1653559299811);
    }
}
