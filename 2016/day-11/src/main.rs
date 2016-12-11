enum Isotope {
    Tm,  // Thulium
    Pu,  // Plutonium,
    Sr,  // Strontium,
    Pm,  // Promethium,
    Ru,  // Ruthenium,
}

enum Item {
    Generator(Isotope),
    Microchip(Isotope),
}

struct Building {
    floors: Vec<Vec<Item>>,
}

fn get_initial_building() -> Building {
    use Isotope::*;
    use Item::*;
    let floors = vec![
        vec![Generator(Tm), Microchip(Tm), Generator(Pu), Generator(Sr)],
        vec![Microchip(Pu), Microchip(Sr)],
        vec![Generator(Pu), Microchip(Pu), Generator(Ru), Microchip(Ru)],
        vec![],
    ];
    Building { floors: floors }
}


fn main() {
    let mut building = get_initial_building();
}
