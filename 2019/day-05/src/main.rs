use intcode::Computer;

fn part1() -> i32 {
    let mut comp = Computer::from_file("input.txt");
    comp.set_input(1);
    comp.execute();
    comp.final_output().unwrap().clone()
}

fn main() {
    println!("Part 1: The final output is {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 13087969);
    }
}
