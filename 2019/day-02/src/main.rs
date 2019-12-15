use intcode::Computer;

fn find_inputs(comp: &mut Computer, target: i32) -> (i32, i32) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut new_comp = comp.clone();
            new_comp.set_noun_verb(noun, verb);
            if new_comp.execute() == target {
                return (noun, verb);
            }
        }
    }
    panic!("Could not find the target output!");
}

fn main() {
    let mut comp = Computer::from_file("input.txt");
    comp.set_noun_verb(12, 2);
    let output = comp.execute();
    println!("Part 1: position 0 --> {}", output);
    let (noun, verb) = find_inputs(&mut comp, 19690720);
    println!("Part 2: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut comp = Computer::from_file("input.txt");
        comp.set_noun_verb(12, 2);
        assert_eq!(comp.execute(), 3409710);
    }

    #[test]
    fn test_part2() {
        let mut comp = Computer::from_file("input.txt");
        let (noun, verb) = find_inputs(&mut comp, 19690720);
        assert_eq!(noun, 79);
        assert_eq!(verb, 12);
    }

}
