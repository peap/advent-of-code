use common::{default_puzzle, Puzzle};
use intcode::{Computer, Val};

type PhaseSettings = Vec<Val>;

fn all_phase_settings(options: PhaseSettings) -> Vec<PhaseSettings> {
    let mut all_settings = vec![];
    if options.len() == 1 {
        return vec![options];
    }
    for i in 0..options.len() {
        let settings = vec![options[i]];
        let mut copy = options.clone();
        copy.remove(i);
        for ext in all_phase_settings(copy) {
            let mut settings = settings.clone();
            settings.extend(ext);
            all_settings.push(settings);
        }
    }
    all_settings
}

fn maximize_amplifiers(comp: Computer, settings: Vec<PhaseSettings>) -> Val {
    let mut maximum = 0;
    for phases in settings {
        let mut last_amp_output = 0;
        let mut last_amp_finished = false;
        let mut computers = vec![];
        for amp_phase in phases {
            let mut new_comp = comp.clone();
            new_comp.set_input(amp_phase);
            new_comp.set_input(last_amp_output);
            new_comp.execute();
            last_amp_output = *new_comp.final_output().unwrap();
            last_amp_finished = new_comp.is_finished();
            computers.push(new_comp);
        }
        while !last_amp_finished {
            for comp in computers.iter_mut() {
                comp.set_input(last_amp_output);
                comp.execute();
                last_amp_output = *comp.final_output().unwrap();
                last_amp_finished = comp.is_finished();
            }
        }
        if last_amp_output > maximum {
            maximum = last_amp_output;
        }
    }
    maximum
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Amplification Circuit");
    puzzle.set_part1("highest thruster signal", |reader| {
        let comp = Computer::from_reader(reader);
        let settings = all_phase_settings((0..5).collect());
        maximize_amplifiers(comp, settings) as u64
    });
    puzzle.set_part2("highest thruster signal (new settings)", |reader| {
        let comp = Computer::from_reader(reader);
        let settings = all_phase_settings((5..10).collect());
        maximize_amplifiers(comp, settings) as u64
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
    fn test_all_phase_settings() {
        assert_eq!(
            all_phase_settings((0..3).collect()),
            vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
            ]
        );
    }

    #[test]
    fn test_example_1() {
        let comp = Computer::new(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        let settings = all_phase_settings((0..5).collect());
        assert_eq!(maximize_amplifiers(comp, settings), 43210);
    }

    #[test]
    fn test_example_2() {
        let comp = Computer::new(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        let settings = all_phase_settings((0..5).collect());
        assert_eq!(maximize_amplifiers(comp, settings), 54321);
    }

    #[test]
    fn test_example_3() {
        let comp = Computer::new(vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
        let settings = all_phase_settings((0..5).collect());
        assert_eq!(maximize_amplifiers(comp, settings), 65210);
    }

    #[test]
    fn test_example_4() {
        let comp = Computer::new(vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);
        let settings = all_phase_settings((5..10).collect());
        assert_eq!(maximize_amplifiers(comp, settings), 139629729);
    }

    #[test]
    fn test_example_5() {
        let comp = Computer::new(vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);
        let settings = all_phase_settings((5..10).collect());
        assert_eq!(maximize_amplifiers(comp, settings), 18216);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(225056);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(14260332);
    }
}
