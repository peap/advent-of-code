#[derive(Debug, Eq, PartialEq)]
enum Isotope {
    Tm,  // Thulium
    Pu,  // Plutonium,
    Sr,  // Strontium,
    Pm,  // Promethium,
    Ru,  // Ruthenium,
}

#[derive(Debug, Eq, PartialEq)]
enum Item {
    Generator(Isotope),
    Microchip(Isotope),
}

impl Item {
    fn compatible_with(&self, other: &Item) -> bool {
        use Item::*;
        match self {
            &Generator(ref iso) => {
                match other {
                    &Generator(_) => true,
                    &Microchip(ref iso2) => iso == iso2,
                }
            }
            &Microchip(ref iso) => {
                match other {
                    &Generator(ref iso2) => iso == iso2,
                    &Microchip(_) => true,
                }
            }
        }
    }
}

struct Floor {
    items: Vec<Item>,
}

impl Floor {
    fn new(items: Vec<Item>) -> Floor {
        Floor { items: items }
    }
}

struct Building {
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
}

fn get_initial_building() -> Building {
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
    let mut building = get_initial_building();
}
