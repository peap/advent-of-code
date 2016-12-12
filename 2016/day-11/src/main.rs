use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Isotope {
    Pm,  // Promethium,
    Pu,  // Plutonium,
    Ru,  // Ruthenium,
    Sr,  // Strontium,
    Tm,  // Thulium
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Item {
    Generator(Isotope),
    Microchip(Isotope),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Floor {
    generators: HashSet<Isotope>,
    microchips: HashSet<Isotope>,
}

impl Hash for Floor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut to_hash = String::new();
        let mut sorted_generators: Vec<Isotope> =
            self.generators.clone().into_iter().collect();
        sorted_generators.sort();
        let mut sorted_microchips: Vec<Isotope> =
            self.microchips.clone().into_iter().collect();
        sorted_microchips.sort();
        for iso in sorted_generators.iter() {
            to_hash.push_str(&format!("{:?}", iso));
        }
        for iso in sorted_microchips.iter() {
            to_hash.push_str(&format!("{:?}", iso));
        }
        to_hash.hash(state)
    }
}

impl Floor {
    fn new(items: Vec<Item>) -> Floor {
        use Item::*;
        let mut generators = HashSet::new();
        let mut microchips = HashSet::new();
        for item in items.iter() {
            match item {
                &Generator(ref iso) => generators.insert(iso.clone()),
                &Microchip(ref iso) => microchips.insert(iso.clone()),
            };
        }
        Floor {
            generators: generators,
            microchips: microchips,
        }
    }

    fn is_safe_for_microchips(&self) -> bool {
        if self.generators.len() == 0 {
            return true;
        }
        for isotope in self.microchips.iter() {
            if !self.generators.contains(isotope) {
                return false;
            }
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.generators.len() == 0 && self.microchips.len() == 0
    }
}

#[derive(Clone, Debug)]
pub struct Building {
    floors: Vec<Floor>,
    height: usize,
    elevator: usize,
    n_moves: u32,
}

impl PartialEq for Building {
    fn eq(&self, other: &Building) -> bool {
        self.floors == other.floors &&
        self.height == other.height &&
        self.elevator == other.elevator
    }
}
impl Eq for Building {}

impl Hash for Building {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // exclude n_moves from the Hash
        for floor in self.floors.iter() {
            floor.hash(state);
        }
        self.height.hash(state);
        self.elevator.hash(state);
    }
}

impl fmt::Display for Building {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+----------------------------+\n").unwrap();
        write!(f, "| Fn  E  Pm  Pu  Ru  Sr  Tm  |\n").unwrap();
        write!(f, "+----------------------------+\n").unwrap();
        let mut reversed_floors = self.floors.clone();
        reversed_floors.reverse();
        for (i, floor) in reversed_floors.iter().enumerate() {
            write!(f, "|  {}", self.height - i).unwrap();
            if self.elevator == self.height - i - 1 {
                write!(f, "  x").unwrap();
            } else {
                write!(f, "   ").unwrap();
            }
            for _ in floor.generators.iter() { write!(f, "  .").unwrap(); }
            for _ in floor.microchips.iter() { write!(f, "  .").unwrap(); }
            write!(f, "\n").unwrap();
        }
        write!(f, "+----------------------------+\n")
    }
}

impl Building {
    fn with_items(items: Vec<Vec<Item>>) -> Building {
        let mut floors = Vec::new();
        let height = items.len();
        for floor_items in items {
            floors.push(Floor::new(floor_items));
        }
        Building {
            floors: floors,
            height: height,
            elevator: 0,
            n_moves: 0,
        }
    }

    fn n_moves(&self) -> u32 {
        self.n_moves
    }

    fn is_safe_for_microchips(&self) -> bool {
        for floor in self.floors.iter() {
            if !floor.is_safe_for_microchips() {
                return false
            }
        }
        true
    }

    fn has_everything_on_top_floor(&self) -> bool {
        for (i, floor) in self.floors.iter().enumerate() {
            if i < (self.height - 1) && !floor.is_empty() {
                return false;
            }
        }
        true
    }

    fn elevator_can_go_down(&self) -> bool {
        self.elevator > 0
    }

    fn elevator_can_go_up(&self) -> bool {
        self.elevator < (self.height - 1)
    }

    fn clone_and_move(&self,
                      item1: Option<Item>,
                      item2: Option<Item>,
                      to: usize) -> Building {
        let mut new = self.clone();
        new.n_moves += 1;
        new.elevator = to;
        match item1 {
            Some(item) => {
                match item {
                    Item::Generator(iso) => {
                        new.floors[self.elevator].generators.remove(&iso);
                        new.floors[to].generators.insert(iso);
                    }
                    Item::Microchip(iso) => {
                        new.floors[self.elevator].microchips.remove(&iso);
                        new.floors[to].microchips.insert(iso);
                    }
                }
            }
            None => (),
        }
        match item2 {
            Some(item) => {
                match item {
                    Item::Generator(iso) => {
                        new.floors[self.elevator].generators.remove(&iso);
                        new.floors[to].generators.insert(iso);
                    }
                    Item::Microchip(iso) => {
                        new.floors[self.elevator].microchips.remove(&iso);
                        new.floors[to].microchips.insert(iso);
                    }
                }
            }
            None => (),
        }
        new
    }

    fn get_next_states(&self) -> Vec<Building> {
        use Item::*;
        let mut states = Vec::new();
        if self.has_everything_on_top_floor() {
            return states;
        }
        let ref elevator_floor = self.floors[self.elevator];
        let mut possible_floors: Vec<usize> = Vec::new();
        let n = self.elevator;
        if self.elevator_can_go_down() {
            states.push(self.clone_and_move(None, None, n - 1));
            possible_floors.push(n - 1);
        }
        if self.elevator_can_go_up() {
            states.push(self.clone_and_move(None, None, n + 1));
            possible_floors.push(n + 1);
        }
        let gens: Vec<Isotope> = elevator_floor.generators.clone().into_iter().collect();
        for (i, iso) in gens.iter().enumerate() {
            let item = Generator(iso.clone());
            for f in possible_floors.iter() {
                let possibility = self.clone_and_move(Some(item), None, *f);
                if possibility.is_safe_for_microchips() {
                    states.push(possibility);
                }
            }
            for j in i..gens.len() {
                let item2 = Generator(gens[j]);
                for f in possible_floors.iter() {
                    let possibility = self.clone_and_move(Some(item),
                                                          Some(item2),
                                                          *f);
                    if possibility.is_safe_for_microchips() {
                        states.push(possibility);
                    }
                }
            }
        }
        let chps: Vec<Isotope> = elevator_floor.microchips.clone().into_iter().collect();
        for (i, iso) in chps.iter().enumerate() {
            let item = Microchip(iso.clone());
            for f in possible_floors.iter() {
                let possibility = self.clone_and_move(Some(item), None, *f);
                if possibility.is_safe_for_microchips() {
                    states.push(possibility);
                }
            }
            for j in i..chps.len() {
                let item2 = Microchip(chps[j]);
                for f in possible_floors.iter() {
                    let possibility = self.clone_and_move(Some(item),
                                                          Some(item2),
                                                          *f);
                    if possibility.is_safe_for_microchips() {
                        states.push(possibility);
                    }
                }
            }
        }
        states
    }
}

pub fn get_example_building() -> Building {
    use Isotope::*;
    use Item::*;
    // From input.txt:
    Building::with_items(vec![
        vec![Microchip(Tm), Microchip(Pu)],
        vec![Generator(Tm)],
        vec![Generator(Pu)],
        vec![],
    ])
}

pub fn get_initial_building() -> Building {
    use Isotope::*;
    use Item::*;
    // From input.txt:
    Building::with_items(vec![
        vec![Generator(Tm), Microchip(Tm), Generator(Pu), Generator(Sr)],
        vec![Microchip(Pu), Microchip(Sr)],
        vec![Generator(Pm), Microchip(Pm), Generator(Ru), Microchip(Ru)],
        vec![],
    ])
}

pub fn minimize_elevator_trips(building: Building) -> u32 {
    if building.has_everything_on_top_floor() {
        return 0;
    }
    let mut min: u32 = u32::max_value();
    let mut bq: VecDeque<Building> = VecDeque::new();
    let mut unique_buildings: HashMap<Building, u32> = HashMap::new();
    // println!("{}", building);
    let next_states = building.get_next_states();
    for bldg in next_states {
        bq.push_back(bldg);
    }
    while !bq.is_empty() {
        // println!("There are {} moves to process.", bq.len());
        let new_building = bq.pop_front().unwrap();
        // println!("{}", new_building);
        if !unique_buildings.contains_key(&new_building) {
            unique_buildings.insert(new_building.clone(), new_building.n_moves());
        }
        if new_building.has_everything_on_top_floor() {
            let num = new_building.n_moves().clone();
            // println!("Finished a building in {} moves!", num);
            if num < min {
                min = num;
            }
        } else {
            let next_states = new_building.get_next_states();
            for bldg in next_states {
                match unique_buildings.get(&bldg) {
                    Some(previous_n_moves) => {
                        if previous_n_moves < &bldg.n_moves() {
                            continue;
                        }
                    }
                    None => (),
                }
                unique_buildings.insert(bldg.clone(), bldg.n_moves());
                bq.push_back(bldg);
            }
        }
    }
    min
}

fn main() {
    use Isotope::*;
    use Item::*;
    let building = Building::with_items(vec![
        vec![Generator(Pu), Microchip(Pu)],
        vec![],
    ]);
    let num_moves = minimize_elevator_trips(building);
    // let building = get_initial_building();
    // let num_moves = minimize_elevator_trips(building);
    println!("Part 1: takes at least {} moves", num_moves);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn get_simple_building() -> Building {
        use Isotope::*;
        use Item::*;
        Building::with_items(vec![
            vec![Generator(Pu), Microchip(Pu)],
            vec![],
        ])
    }

    #[test]
    fn test_building_methods() {
        use Isotope::*;
        use Item::*;
        // make a simple building
        let building = get_simple_building();
        assert_eq!(building.n_moves(), 0);
        assert!(building.is_safe_for_microchips());
        assert!(!building.has_everything_on_top_floor());
        assert!(!building.elevator_can_go_down());
        assert!(building.elevator_can_go_up());
        // move elevator up
        let building = building.clone_and_move(None, None, 1);
        assert_eq!(building.n_moves(), 1);
        assert!(building.is_safe_for_microchips());
        assert!(!building.has_everything_on_top_floor());
        assert!(building.elevator_can_go_down());
        assert!(!building.elevator_can_go_up());
        // move elevator back down
        let building = building.clone_and_move(None, None, 0);
        assert_eq!(building.n_moves(), 2);
        assert!(building.is_safe_for_microchips());
        assert!(!building.has_everything_on_top_floor());
        assert!(!building.elevator_can_go_down());
        assert!(building.elevator_can_go_up());
        // move generator up
        let building = building.clone_and_move(Some(Generator(Pu)), None, 1);
        assert_eq!(building.n_moves(), 3);
        assert!(building.is_safe_for_microchips());
        assert!(!building.has_everything_on_top_floor());
        assert!(building.elevator_can_go_down());
        assert!(!building.elevator_can_go_up());
        // move elevator back down
        let building = building.clone_and_move(None, None, 0);
        assert_eq!(building.n_moves(), 4);
        assert!(building.is_safe_for_microchips());
        assert!(!building.has_everything_on_top_floor());
        assert!(!building.elevator_can_go_down());
        assert!(building.elevator_can_go_up());
        // move microchip up
        let building = building.clone_and_move(Some(Microchip(Pu)), None, 1);
        assert_eq!(building.n_moves(), 5);
        assert!(building.is_safe_for_microchips());
        assert!(building.has_everything_on_top_floor());
        assert!(building.elevator_can_go_down());
        assert!(!building.elevator_can_go_up());
    }

    #[test]
    fn test_building_hash() {
        use Isotope::*;
        use Item::*;
        let mut set = HashSet::new();
        let building1 = Building::with_items(vec![
            vec![Generator(Pu), Microchip(Pu)],
            vec![],
        ]);
        set.insert(building1);
        let building2 = Building::with_items(vec![
            vec![Microchip(Pu), Generator(Pu)],
            vec![],
        ]);
        assert!(set.contains(&building2));
        let building3 = Building::with_items(vec![
            vec![],
            vec![Microchip(Pu), Generator(Pu)],
        ]);
        assert!(!set.contains(&building3));
    }

    #[test]
    fn test_building_hash_with_moves() {
        use Isotope::*;
        use Item::*;
        let mut set = HashSet::new();
        let building1 = Building::with_items(vec![
            vec![Generator(Pu), Microchip(Pu)],
            vec![],
        ]);
        set.insert(building1.clone());
        // move generator up
        let building2 = building1.clone_and_move(Some(Generator(Pu)), None, 1);
        assert!(!set.contains(&building2));
        // move generator back down
        let building3 = building2.clone_and_move(Some(Generator(Pu)), None, 0);
        assert!(set.contains(&building3));
    }

    #[test]
    fn test_single_story_building() {
        use Isotope::*;
        use Item::*;
        let building = Building::with_items(vec![
            vec![Generator(Pu), Microchip(Pu)],
        ]);
        assert!(building.has_everything_on_top_floor());
        let num = minimize_elevator_trips(building);
        assert_eq!(num, 0);
    }

    #[test]
    fn test_two_story_building_already_done() {
        use Isotope::*;
        use Item::*;
        let building = Building::with_items(vec![
            vec![],
            vec![Generator(Pu)],
        ]);
        assert!(building.has_everything_on_top_floor());
        let num = minimize_elevator_trips(building);
        assert_eq!(num, 0);
    }

    #[test]
    fn test_two_story_building_with_one_move_to_go() {
        use Isotope::*;
        use Item::*;
        let building = Building::with_items(vec![
            vec![Generator(Pu)],
            vec![],
        ]);
        assert!(!building.has_everything_on_top_floor());
        let num = minimize_elevator_trips(building);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_two_story_building_with_one_move_to_go_and_two_items() {
        use Isotope::*;
        use Item::*;
        let building = Building::with_items(vec![
            vec![Generator(Pu)],
            vec![Microchip(Pu)],
        ]);
        assert!(!building.has_everything_on_top_floor());
        let num = minimize_elevator_trips(building);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_three_story_building_with_two_moves_to_go() {
        use Isotope::*;
        use Item::*;
        let building = Building::with_items(vec![
            vec![],
            vec![Generator(Pu)],
            vec![],
        ]);
        assert!(!building.has_everything_on_top_floor());
        let num = minimize_elevator_trips(building);
        assert_eq!(num, 2);
    }

    #[test]
    fn test_stupid_simple_building() {
        use Isotope::*;
        use Item::*;
        let building = Building::with_items(vec![
            vec![Generator(Pu), Microchip(Pu)],
            vec![],
        ]);
        let num_moves = minimize_elevator_trips(building);
        assert_eq!(num_moves, 3);
    }

    #[test]
    #[ignore]
    fn test_example_building() {
        let building = get_example_building();
        let num_moves = minimize_elevator_trips(building);
        assert_eq!(num_moves, 11);
    }

    #[test]
    fn test_initial_building_is_safe() {
        let building = get_initial_building();
        assert!(building.is_safe_for_microchips());
    }

    #[test]
    #[ignore]
    fn test_part_1_answer() {
        let building = get_initial_building();
        let num_moves = minimize_elevator_trips(building);
        assert_eq!(num_moves, 25); // TODO: find answer :)
    }
}
