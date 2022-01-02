use std::cmp;

use common::{default_puzzle, Puzzle};

const BOSS_HP: i64 = 103;
const BOSS_DAMAGE: i64 = 9;
const BOSS_ARMOR: i64 = 2;

// (cost, damage, armor)
type Item = (i64, i64, i64);

const WEAPONS: [Item; 5] = [
    (8, 4, 0),  // Dagger
    (10, 5, 0), // Shortsword
    (25, 6, 0), // Warhammer
    (40, 7, 0), // Longsword
    (74, 8, 0), // Greataxe
];

const ARMORS: [Item; 6] = [
    (13, 0, 1),  // Leather
    (31, 0, 2),  // Chainmail
    (53, 0, 3),  // Splintmail
    (75, 0, 4),  // Bandedmail
    (102, 0, 5), // Platemail
    (0, 0, 0),   // i.e., no armor
];

const RINGS: [Item; 7] = [
    (25, 1, 0),  // Damage +1
    (50, 2, 0),  // Damage +2
    (100, 3, 0), // Damage +3
    (20, 0, 1),  // Defense +1
    (40, 0, 2),  // Defense +2
    (80, 0, 3),  // Defense +3
    (0, 0, 0),   // i.e., no rings
];

#[derive(Clone)]
pub struct Combatant {
    hp: i64,
    damage: i64,
    armor: i64,
    gold: u64,
}

impl Combatant {
    fn new_player(weapon: &Item, armor: &Item, rings: Vec<&Item>) -> Self {
        let mut damage = weapon.1;
        let mut armor_pts = armor.2;
        let mut gold = 0;
        gold += weapon.0;
        gold += armor.0;
        for ring in rings.iter() {
            gold += ring.0;
            damage += ring.1;
            armor_pts += ring.2;
        }
        Combatant {
            hp: 100,
            damage,
            armor: armor_pts,
            gold: gold as u64,
        }
    }

    fn new_boss(hp: i64, damage: i64, armor: i64) -> Self {
        Combatant {
            hp,
            damage,
            armor,
            gold: 0,
        }
    }

    fn defeats(&mut self, enemy: &mut Combatant) -> bool {
        let my_damage = cmp::max(1, self.damage - enemy.armor);
        let enemy_damage = cmp::max(1, enemy.damage - self.armor);
        while self.hp > 0 && enemy.hp > 0 {
            enemy.hp -= my_damage;
            if enemy.hp <= 0 {
                return true;
            }
            self.hp -= enemy_damage;
            if self.hp <= 0 {
                return false;
            }
        }
        self.hp > enemy.hp
    }
}

pub fn min_and_max_to_defeat_and_lose() -> (u64, u64) {
    let mut minimum_gold: u64 = u64::max_value();
    let mut maximum_gold: u64 = u64::min_value();
    let boss = Combatant::new_boss(BOSS_HP, BOSS_DAMAGE, BOSS_ARMOR);
    for weapon in WEAPONS.iter() {
        for armor in ARMORS.iter() {
            for (i, ring1) in RINGS.iter().enumerate() {
                let rings1 = vec![ring1];
                let mut boss1 = boss.clone();
                let mut player1 = Combatant::new_player(weapon, armor, rings1);
                if player1.defeats(&mut boss1) {
                    minimum_gold = cmp::min(minimum_gold, player1.gold);
                } else {
                    maximum_gold = cmp::max(maximum_gold, player1.gold);
                }
                for ring2 in RINGS[(i + 1)..].iter() {
                    let rings2 = vec![ring1, ring2];
                    let mut boss2 = boss.clone();
                    let mut player2 = Combatant::new_player(weapon, armor, rings2);
                    if player2.defeats(&mut boss2) {
                        minimum_gold = cmp::min(minimum_gold, player2.gold);
                    } else {
                        maximum_gold = cmp::max(maximum_gold, player2.gold);
                    }
                }
            }
        }
    }
    (minimum_gold, maximum_gold)
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("RPG Simulator 20XX");
    puzzle.set_part1("least gold to win the fight", |_| {
        min_and_max_to_defeat_and_lose().0
    });
    puzzle.set_part2("most gold to lose the fight", |_| {
        min_and_max_to_defeat_and_lose().1
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
    fn test_example() {
        let mut player = Combatant {
            hp: 8,
            damage: 5,
            armor: 5,
            gold: 0,
        };
        let mut boss = Combatant {
            hp: 12,
            damage: 7,
            armor: 2,
            gold: 0,
        };
        assert!(player.defeats(&mut boss));
    }

    #[test]
    fn test_parts1() {
        get_puzzle().test_part1(121);
    }

    #[test]
    fn test_parts2() {
        get_puzzle().test_part2(201);
    }
}
