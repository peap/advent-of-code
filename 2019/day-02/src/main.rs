use common::{default_puzzle, Puzzle};
use intcode::{Computer, Val};

fn find_inputs(comp: &mut Computer, target: Val) -> (Val, Val) {
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

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("1202 Program Alarm");
    puzzle.set_part1("position 0", |reader| {
        // let mut comp = Computer::from_file("input.txt");
        let mut comp = Computer::from_reader(reader);
        comp.set_noun_verb(12, 2);
        comp.execute() as u64
    });
    puzzle.set_part2("100 * noun + verb", |reader| {
        // let mut comp = Computer::from_file("input.txt");
        let mut comp = Computer::from_reader(reader);
        let (noun, verb) = find_inputs(&mut comp, 19690720);
        (100 * noun + verb) as u64
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
    fn test_part1() {
        get_puzzle().test_part1(3409710);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(7912);
    }
}
