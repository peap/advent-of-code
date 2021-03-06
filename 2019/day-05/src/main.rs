use intcode::{Computer, Val};

fn part1() -> Val {
    let mut comp = Computer::from_file("input.txt");
    comp.set_input(1);
    comp.execute();
    comp.final_output().unwrap().clone()
}

fn part2() -> Val {
    let mut comp = Computer::from_file("input.txt");
    comp.set_input(5);
    comp.execute();
    comp.final_output().unwrap().clone()
}

fn main() {
    println!("Part 1: The final output is {}", part1());
    println!("Part 2: The final output is {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 13087969);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 14110739);
    }
}
