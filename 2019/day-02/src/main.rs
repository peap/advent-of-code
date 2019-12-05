use intcode::Computer;

fn find_inputs(comp: Computer, target: i32) -> (i32, i32) {
    for noun in 0..100 {
        for verb in 0..100 {
            if comp.execute(noun, verb) == target {
                return (noun, verb);
            }
        }
    }
    panic!("Could not find the target output!");
}

fn main() {
    let comp = Computer::from_file("input.txt");
    let output = comp.execute(12, 2);
    println!("Part 1: position 0 --> {}", output);
    let (noun, verb) = find_inputs(comp, 19690720);
    println!("Part 2: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let comp = Computer::from_file("input.txt");
        assert_eq!(comp.execute(12, 2), 3409710);
    }

    #[test]
    fn test_part2() {
        let comp = Computer::from_file("input.txt");
        let (noun, verb) = find_inputs(comp, 19690720);
        assert_eq!(noun, 79);
        assert_eq!(verb, 12);
    }

}
