use common::{default_puzzle, Puzzle};

fn linear_cost(distance: u64) -> u64 {
    distance
}

fn triangle_cost(distance: u64) -> u64 {
    (distance * (distance + 1)) / 2
}

fn expand_from_median(nums: &[u64]) -> Vec<u64> {
    let min = *nums.first().unwrap();
    let median = nums[nums.len() / 2];
    let max = *nums.last().unwrap();
    let mut expanded = vec![];
    let (mut left, mut right) = (median, median + 1);
    while expanded.len() < (max as usize + 1 - min as usize) {
        if left >= min {
            expanded.push(left);
        }
        if right <= max {
            expanded.push(right);
        }
        if left > min {
            left -= 1;
        }
        if right < max {
            right += 1;
        }
    }
    expanded
}

fn minimize_moves(crabs: Vec<u64>, cost_fn: &dyn Fn(u64) -> u64) -> u64 {
    let positions = expand_from_median(&crabs);
    let mut best_cost = u64::MAX;
    for pos in positions.iter() {
        let mut cost = 0;
        for crab in crabs.iter() {
            let distance = if *crab >= *pos {
                crab - *pos
            } else {
                *pos - crab
            };
            cost += cost_fn(distance);
            if cost > best_cost {
                break;
            }
        }
        if cost < best_cost {
            best_cost = cost;
        }
    }
    best_cost
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("The Treachery of Whales");
    puzzle.set_part1("best position costs (linear)", |reader| {
        let mut crabs = reader.parsed_csv_line();
        crabs.sort_unstable();
        minimize_moves(crabs, &linear_cost)
    });
    puzzle.set_part2("best position costs (triangle)", |reader| {
        let mut crabs = reader.parsed_csv_line();
        crabs.sort_unstable();
        minimize_moves(crabs, &triangle_cost)
    });
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut crabs: Vec<u64> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        crabs.sort_unstable();
        assert_eq!(minimize_moves(crabs.clone(), &linear_cost), 37);
        assert_eq!(minimize_moves(crabs, &triangle_cost), 168);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(339321);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(95476244);
    }
}
