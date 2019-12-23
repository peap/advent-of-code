use intcode::{Computer, Val};

fn part1() -> Val {
    let mut comp = Computer::from_file("input.txt");
    comp.set_input(1);
    comp.execute();
    comp.final_output().unwrap().clone()
}

fn main() {
    println!("Part1: The BOOST code keycode is {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2399197539);
    }
}
