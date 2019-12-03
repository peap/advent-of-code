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
    Elerium,
    Dilithium,
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
        self.get_iso_pairs() == other.get_iso_pairs() &&
        self.height == other.height &&
        self.elevator == other.elevator
    }
}
impl Eq for Building {}

impl Hash for Building {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_iso_pairs().hash(state);
        self.height.hash(state);
        self.elevator.hash(state);
        // exclude n_moves from the Hash
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

    fn find_item(&self, item: Item) -> Option<usize> {
        for (i, floor) in self.floors.iter().enumerate() {
            match item {
                Item::Generator(iso) => {
                    if floor.generators.contains(&iso) {
                        return Some(i);
                    }
                }
                Item::Microchip(iso) => {
                    if floor.microchips.contains(&iso) {
                        return Some(i);
                    }
                }
            }
        }
        None
    }

    fn get_iso_pairs(&self) -> Vec<Vec<usize>> {
        use Isotope::*;
        use Item::*;
        // Insight from https://andars.github.io/aoc_day11.html: it's only the
        // locations of the pairs of the generators and microchips that
        // matters, not the particulars of which isotope is where. So, add them
        // to the building hash as pairs.
        let mut iso_pairs: Vec<Vec<usize>> = Vec::new();
        for iso in vec![Pm, Pu, Ru, Sr, Tm, Elerium, Dilithium] {
            let generator_floor = self.find_item(Generator(iso));
            let microchip_floor = self.find_item(Microchip(iso));
            if generator_floor.is_some() && microchip_floor.is_some() {
                let mut pair_floors: Vec<usize> = vec![
                    generator_floor.unwrap(),
                    microchip_floor.unwrap(),
                ];
                pair_floors.sort();
                iso_pairs.push(pair_floors);
            }
        }
        iso_pairs.sort();
        iso_pairs
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
        let ref curr_floor = self.floors[self.elevator];
        let mut possible_floors: Vec<usize> = Vec::new();
        if self.elevator_can_go_up() {
            possible_floors.push(self.elevator + 1);
        }
        if self.elevator_can_go_down() {
            possible_floors.push(self.elevator - 1);
        }
        {
            let mut add_state = |item1, item2| {
                for f in possible_floors.iter() {
                    let possibility = self.clone_and_move(item1, item2, *f);
                    if possibility.is_safe_for_microchips() {
                        states.push(possibility);
                    }
                }
            };
            let gens: Vec<_> = curr_floor.generators.clone().into_iter().collect();
            let chps: Vec<_> = curr_floor.microchips.clone().into_iter().collect();
            for (i, iso) in gens.iter().enumerate() {
                let item = Some(Generator(iso.clone()));
                add_state(item, None);
                if chps.contains(&iso) {
                    add_state(item, Some(Microchip(iso.clone())));
                }
                for j in (i+1)..gens.len() {
                    let item2 = Some(Generator(gens[j].clone()));
                    add_state(item, item2);
                }
            }
            for (i, iso) in chps.iter().enumerate() {
                let item = Some(Microchip(iso.clone()));
                add_state(item, None);
                for j in (i+1)..chps.len() {
                    let item2 = Some(Microchip(chps[j].clone()));
                    add_state(item, item2);
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

pub fn get_initial_building_for_part_two() -> Building {
    use Isotope::*;
    use Item::*;
    // From input.txt:
    Building::with_items(vec![
        vec![Generator(Tm), Microchip(Tm), Generator(Pu), Generator(Sr),
             Generator(Elerium), Microchip(Elerium),
             Generator(Dilithium), Microchip(Dilithium)],
        vec![Microchip(Pu), Microchip(Sr)],
        vec![Generator(Pm), Microchip(Pm), Generator(Ru), Microchip(Ru)],
        vec![],
    ])
}

pub fn minimize_elevator_trips(building: Building) -> u32 {
    if building.has_everything_on_top_floor() {
        return 0;
    }
    let max_queue = 3_000_000;
    let mut bq: VecDeque<Building> = VecDeque::new();
    let mut unique_buildings: HashMap<Building, u32> = HashMap::new();
    let next_states = building.get_next_states();
    for bldg in next_states {
        bq.push_back(bldg);
    }
    while !bq.is_empty() {
        if bq.len() > max_queue {
            println!("Bailing out with {} moves left to process.", max_queue);
            break;
        }
        let new_building = bq.pop_front().unwrap();
        if bq.len() % 250000 < 5 {
            println!("There are {} moves to process; working on move #{}.",
                     bq.len(),
                     new_building.n_moves());
        }
        if !unique_buildings.contains_key(&new_building) {
            unique_buildings.insert(new_building.clone(), new_building.n_moves());
        }
        if new_building.has_everything_on_top_floor() {
            // doing BFS, so this is the answer
            return new_building.n_moves();
        } else {
            let next_states = new_building.get_next_states();
            for bldg in next_states {
                match unique_buildings.get(&bldg) {
                    Some(_) => continue,
                    None => (),
                }
                unique_buildings.insert(bldg.clone(), bldg.n_moves());
                bq.push_back(bldg);
            }
        }
    }
    u32::max_value()
}

fn main() {
//    // the example building
//    let building = get_example_building();
//    let num_moves = minimize_elevator_trips(building);
    // my building
    let building = get_initial_building();
    let num_moves = minimize_elevator_trips(building);
    println!("Part 1: takes at least {} moves", num_moves);
    // my building, part 2
    let building = get_initial_building_for_part_two();
    let num_moves = minimize_elevator_trips(building);
    println!("Part 2: takes at least {} moves", num_moves);
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
    fn test_building_hash_just_cares_about_pairs() {
        use Isotope::*;
        use Item::*;
        let mut set = HashSet::new();
        let building1 = Building::with_items(vec![
            vec![Generator(Pu)],
            vec![Microchip(Pu)],
        ]);
        set.insert(building1);
        let building2 = Building::with_items(vec![
            vec![Microchip(Pu)],
            vec![Generator(Pu)],
        ]);
        assert!(set.contains(&building2));
        let building3 = Building::with_items(vec![
            vec![Microchip(Tm)],
            vec![Generator(Tm)],
        ]);
        assert!(set.contains(&building3));
        let building4 = Building::with_items(vec![
            vec![Generator(Tm)],
            vec![Microchip(Tm)],
        ]);
        assert!(set.contains(&building4));
    }

    #[test]
    fn test_building_hash_just_cares_about_pairs_with_two_isotopes() {
        use Isotope::*;
        use Item::*;
        let mut set = HashSet::new();
        let building1 = Building::with_items(vec![
            vec![Generator(Pu), Generator(Tm)],
            vec![Microchip(Pu), Microchip(Tm)],
        ]);
        set.insert(building1);
        let building2 = Building::with_items(vec![
            vec![Microchip(Pu), Microchip(Tm)],
            vec![Generator(Pu), Generator(Tm)],
        ]);
        assert!(set.contains(&building2));
        let building3 = Building::with_items(vec![
            vec![Microchip(Pm), Microchip(Sr)],
            vec![Generator(Pm), Generator(Sr)],
        ]);
        assert!(set.contains(&building3));
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
            vec![Generator(Pu)],
            vec![],
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
        assert_eq!(num_moves, 1);
    }

    #[test]
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
    fn test_part_1_answer() {
        let building = get_initial_building();
        let num_moves = minimize_elevator_trips(building);
        assert_eq!(num_moves, 31);
    }

    #[test]
    fn test_part_2_answer() {
        let building = get_initial_building_for_part_two();
        let num_moves = minimize_elevator_trips(building);
        assert_eq!(num_moves, 55);
    }
}
