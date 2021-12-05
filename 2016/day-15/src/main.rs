#[macro_use]
extern crate lazy_static;

#[derive(Clone)]
pub struct Disc {
    id: u32,
    n_pos: u32,
    start_pos: u32,
}

impl Disc {
    fn new(id: u32, n_pos: u32, start_pos: u32) -> Disc {
        Disc {
            id: id,
            n_pos: n_pos,
            start_pos: start_pos,
        }
    }

    fn get_position_at(&self, time: u32) -> u32 {
        (self.start_pos + time) % self.n_pos
    }

    fn would_accept(&self, time: u32) -> bool {
        self.get_position_at(time + self.id) == 0
    }
}

lazy_static! {
    pub static ref DISCS: Vec<Disc> = {
        vec![
            Disc::new(1, 13, 1),
            Disc::new(2, 19, 10),
            Disc::new(3, 3, 2),
            Disc::new(4, 7, 1),
            Disc::new(5, 5, 3),
            Disc::new(6, 17, 5),
        ]
    };
    pub static ref DISCS2: Vec<Disc> = {
        let mut discs2 = DISCS.clone();
        discs2.push(Disc::new(7, 11, 0));
        discs2
    };
}

fn capsule_reaches_bottom<'a>(discs: &'a Vec<Disc>, time: u32) -> bool {
    discs
        .iter()
        .fold(true, |acc, disc| acc && disc.would_accept(time))
}

pub fn get_first_drop_window<'a>(discs: &'a Vec<Disc>) -> Option<u32> {
    (0..).find(|t| capsule_reaches_bottom(discs, *t))
}

fn main() {
    if let Some(time1) = get_first_drop_window(&DISCS) {
        println!("Part 1: earliest time to drop is {}", time1);
    }
    if let Some(time2) = get_first_drop_window(&DISCS2) {
        println!("Part 2: earliest time to drop is {}", time2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let time = get_first_drop_window(&DISCS);
        assert_eq!(time, Some(376777));
    }

    #[test]
    fn test_part_2() {
        let time = get_first_drop_window(&DISCS2);
        assert_eq!(time, Some(3903937));
    }
}
