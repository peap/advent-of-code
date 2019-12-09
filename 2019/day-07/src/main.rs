use std::ops:Range;
use intcode::{Computer, Val};

type PhaseSettings = Vec<Val>;

fn to_phases(int_phase: Val, max: Val, items: usize) -> PhaseSettings {
    let mut
    let mut phases = vec![];
    for _ in 0..items {
        phases.push(0);
    }
    let mut remainder = int_phase;
    for i in 0..items {
        phases[i] = remainder % max;
        remainder /= max;
    }
    phases
}

fn all_phase_settings(range: Range) -> Vec<PhaseSettings> {
    let mut settings = vec![];
    let items: Vec<Val> = range.collect();
    // ...
    settings.push(to_phases(range));
    settings
}

// fn all_phase_settings(max: Val, items: usize) -> Vec<PhaseSettings> {
//     let mut settings = vec![];
//     for n in 0..(max + 1).pow(items as u32) {
//         settings.push(to_phases(n, max + 1, items));
//     }
//     settings
// }

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
    let settings = all_phase_settings(0..5);
    maximize_amplifiers(comp, settings)
}

fn main() {
    println!("Part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_phases() {
        assert_eq!(to_phases(0, 3, 3), vec![0, 0, 0]);
        assert_eq!(to_phases(1, 3, 3), vec![1, 0, 0]);
        assert_eq!(to_phases(2, 3, 3), vec![2, 0, 0]);
        assert_eq!(to_phases(3, 3, 3), vec![0, 1, 0]);
        assert_eq!(to_phases(4, 3, 3), vec![1, 1, 0]);
        assert_eq!(to_phases(5, 3, 3), vec![2, 1, 0]);
        assert_eq!(to_phases(6, 3, 3), vec![0, 2, 0]);
        assert_eq!(to_phases(9, 3, 3), vec![0, 0, 1]);
        assert_eq!(to_phases(10, 3, 3), vec![1, 0, 1]);
        assert_eq!(to_phases(11, 3, 3), vec![2, 0, 1]);
    }

    #[test]
    fn test_example_1() {
        let comp = Computer::new(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        let settings = all_phase_settings(4, 5);
        assert_eq!(maximize_amplifiers(comp, settings), 43210);
    }

    #[test]
    fn test_example_2() {
        let comp = Computer::new(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        let settings = all_phase_settings(4, 5);
        assert_eq!(maximize_amplifiers(comp, settings), 54321);
    }

    #[test]
    fn test_example_3() {
        let comp = Computer::new(vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
        let settings = all_phase_settings(4, 5);
        assert_eq!(maximize_amplifiers(comp, settings), 65210);
    }
}
