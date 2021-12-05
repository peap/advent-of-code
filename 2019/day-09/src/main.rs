use intcode::{Computer, Val};

fn part1() -> Val {
    let mut comp = Computer::from_file("input.txt");
    comp.set_input(1);
    comp.execute();
    *comp.final_output().unwrap()
}

fn part2() -> Val {
    let mut comp = Computer::from_file("input.txt");
    comp.set_input(2);
    comp.execute();
    *comp.final_output().unwrap()
}

fn main() {
    println!("Part 1: The BOOST code keycode is {}", part1());
    println!("Part 2: The distress signal coordinates are {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2399197539);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 35106);
    }
}
