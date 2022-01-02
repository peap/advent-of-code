use common::{default_puzzle, Puzzle};
use intcode::Computer;

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Sensor Boost");
    puzzle.set_part1("BOOST code keycode", |reader| {
        let mut comp = Computer::from_reader(reader);
        comp.set_input(1);
        comp.execute();
        *comp.final_output().unwrap() as u64
    });
    puzzle.set_part2("distress signal coordinates", |reader| {
        let mut comp = Computer::from_reader(reader);
        comp.set_input(2);
        comp.execute();
        *comp.final_output().unwrap() as u64
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
        get_puzzle().test_part1(2399197539);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(35106);
    }
}
