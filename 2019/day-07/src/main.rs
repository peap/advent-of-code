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
        for amp_phase in phases {
            let mut new_comp = comp.clone();
            new_comp.set_input(amp_phase);
            new_comp.set_input(last_amp_output);
            new_comp.execute();
            last_amp_output = new_comp.final_output().unwrap().clone();
        }
        if last_amp_output > maximum {
            maximum = last_amp_output;
        }
    }
    maximum
}

fn part1() -> Val {
    let comp = Computer::from_file("input.txt");
    let settings = all_phase_settings((0..5).collect());
    maximize_amplifiers(comp, settings)
}

fn main() {
    println!("Part 1: The maximum output is {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_phase_settings() {
        assert_eq!(all_phase_settings((0..3).collect()), vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ]);
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
    fn test_part1() {
        assert_eq!(part1(), 225056);
    }
}
