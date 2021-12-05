use std::cmp;

const BOSS_HP: i32 = 103;
const BOSS_DAMAGE: i32 = 9;
const BOSS_ARMOR: i32 = 2;

// (cost, damage, armor)
type Item = (i32, i32, i32);

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
    hp: i32,
    damage: i32,
    armor: i32,
    gold: i32,
}

impl Combatant {
    fn new_player<'a>(weapon: &'a Item, armor: &'a Item, rings: Vec<&'a Item>) -> Self {
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
            damage: damage,
            armor: armor_pts,
            gold: gold,
        }
    }

    fn new_boss(hp: i32, damage: i32, armor: i32) -> Self {
        Combatant {
            hp: hp,
            damage: damage,
            armor: armor,
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

pub fn min_and_max_to_defeat_and_lose() -> (i32, i32) {
    let mut minimum_gold: i32 = i32::max_value();
    let mut maximum_gold: i32 = i32::min_value();
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

fn main() {
    let (min, max) = min_and_max_to_defeat_and_lose();
    println!("Part 1: It costs at least {} gold to defeat the boss.", min);
    println!("Part 2: It costs at most {} gold to lose to the boss.", max);
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
    fn test_parts_1_and_2() {
        let (min, max) = min_and_max_to_defeat_and_lose();
        assert_eq!(min, 121);
        assert_eq!(max, 201);
    }
}
