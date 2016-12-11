use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Isotope {
    Tm,  // Thulium
    Pu,  // Plutonium,
    Sr,  // Strontium,
    Pm,  // Promethium,
    Ru,  // Ruthenium,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Item {
    Generator(Isotope),
    Microchip(Isotope),
}

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
}

pub struct Building {
    floors: Vec<Floor>,
}

impl Building {
    fn with_items(items: Vec<Vec<Item>>) -> Building {
        let mut floors = Vec::new();
        for floor_items in items {
            floors.push(Floor::new(floor_items));
        }
        Building { floors: floors }
    }

    fn is_safe_for_microchips(&self) -> bool {
        for floor in self.floors.iter() {
            if !floor.is_safe_for_microchips() {
                return false
            }
        }
        true
    }
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


fn main() {
    let building = get_initial_building();
    println!("Initial building safe: {}", building.is_safe_for_microchips());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_building_is_safe() {
        let building = get_initial_building();
        assert!(building.is_safe_for_microchips());
    }
}
