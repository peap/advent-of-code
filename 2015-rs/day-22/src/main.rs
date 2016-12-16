use std::cmp;
use std::collections::VecDeque;

pub const BOSS_HP: i32 = 71;
pub const BOSS_DAMAGE: i32 = 10;
pub const PLAYER_HP: i32 = 50;
pub const PLAYER_MANA: i32 = 500;

const MANA_MAGIC_MISSILE: i32 = 53;
const MANA_DRAIN: i32 = 73;
const MANA_SHIELD: i32 = 113;
const MANA_POISON: i32 = 173;
const MANA_RECHARGE: i32 = 229;

#[derive(Clone)]
pub enum Mode {
    Easy,
    Hard,
}

#[derive(Clone, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
    BossAttack,
    Nothing,
}

#[derive(Clone)]
pub struct Combatant {
    hp: i32,
    damage: i32,
    armor: i32,
    mana: i32,
    expenses: i32,
    spell: Spell,
    mode: Mode,
    t_shielded: i32,
    t_poisoned: i32,
    t_recharing: i32,
}

impl Combatant {
    fn new(hp: i32, damage: i32, mana: i32, spell: Spell, mode: Mode) -> Combatant {
        Combatant {
            hp: hp,
            damage: damage,
            armor: 0,
            mana: mana,
            expenses: 0,
            spell: spell,
            mode: mode,
            t_shielded: 0,
            t_poisoned: 0,
            t_recharing: 0,
        }
    }

    fn new_player(hp: i32, mana: i32, mode: Mode) -> Combatant {
        Combatant::new(hp, 0, mana, Spell::Nothing, mode)
    }

    fn new_boss(hp: i32, damage: i32) -> Combatant {
        Combatant::new(hp, damage, 0, Spell::BossAttack, Mode::Easy)
    }

    fn buy(&mut self, mana: i32) {
        self.mana -= mana;
        self.expenses += mana;
    }

    fn use_spell(&mut self, enemy: &mut Combatant) {
        use Spell::*;
        match self.spell {
            MagicMissile => {
                self.buy(MANA_MAGIC_MISSILE);
                enemy.hp -= 4;
            }
            Drain => {
                self.buy(MANA_DRAIN);
                enemy.hp -= 2;
                self.hp += 2;
            }
            Shield => {
                self.buy(MANA_SHIELD);
                self.t_shielded = 6;
            }
            Poison => {
                self.buy(MANA_POISON);
                enemy.t_poisoned = 6;
            }
            Recharge => {
                self.buy(MANA_RECHARGE);
                self.t_recharing = 5;
            }
            BossAttack => {
                enemy.hp -= cmp::max(1, self.damage - enemy.armor);
            }
            Nothing => (),
        }
    }

    fn apply_effects(&mut self, is_my_turn: bool) {
        if is_my_turn {
            match self.mode {
                Mode::Easy => (),
                Mode::Hard => self.hp -= 1,
            }
        }
        if self.t_shielded > 0 {
            self.t_shielded -= 1;
            self.armor = 7;
        } else {
            self.armor = 0;
        }
        if self.t_poisoned > 0 {
            self.t_poisoned -= 1;
            self.hp -= 3;
        }
        if self.t_recharing > 0 {
            self.t_recharing -= 1;
            self.mana += 101;  // doesn't subtract from expenses
        }
    }

    fn clone_with_spell(&self, spell: Spell) -> Combatant {
        let mut cloned = self.clone();
        cloned.spell = spell;
        cloned
    }

    fn get_next_spells(&self, enemy: &Combatant) -> Vec<Combatant> {
        use Spell::*;
        let mut states = vec![];
        let mana = self.mana + if self.t_recharing > 0 { 101 } else { 0 };
        if mana >= MANA_MAGIC_MISSILE {
            states.push(self.clone_with_spell(MagicMissile));
        }
        if mana >= MANA_DRAIN {
            states.push(self.clone_with_spell(Drain));
        }
        if mana >= MANA_SHIELD && self.t_shielded <= 1 {
            states.push(self.clone_with_spell(Shield));
        }
        if mana >= MANA_POISON && enemy.t_poisoned <= 1 {
            states.push(self.clone_with_spell(Poison));
        }
        if mana >= MANA_RECHARGE && self.t_recharing <= 1 {
            states.push(self.clone_with_spell(Recharge));
        }
        states
    }

    fn dead(&self) -> bool {
        self.hp <= 0
    }
}

pub fn find_minimum_mana_to_win(player: Combatant, boss: Combatant) -> i32 {
    let mut minimum_mana = i32::max_value();
    let mut q: VecDeque<(Combatant, Combatant)> = VecDeque::new();
    for p in player.get_next_spells(&boss).iter() {
        q.push_back((p.clone(), boss.clone()));
    }
    while !q.is_empty() {
        if q.len() % 1000 == 0 {
            print!("\rQueue size: {:<9}", q.len());
        }
        let (ref mut player, ref mut boss) = q.pop_front().unwrap();
        // Player turn
        player.apply_effects(true);
        if player.dead() { continue; }
        boss.apply_effects(false);
        player.use_spell(boss);
        if boss.dead() {
            minimum_mana = cmp::min(minimum_mana, player.expenses);
            continue;
        }
        // Boss turn
        boss.apply_effects(true);
        if boss.dead() {
            minimum_mana = cmp::min(minimum_mana, player.expenses);
            continue;
        }
        player.apply_effects(false);
        boss.use_spell(player);
        if player.dead() { continue; }
        // Get next moves
        for p in player.get_next_spells(&boss) {
            q.push_back((p.clone(), boss.clone()));
        }
    }
    minimum_mana
}

fn main() {
    // Part 1
    let player = Combatant::new_player(PLAYER_HP, PLAYER_MANA, Mode::Easy);
    let boss = Combatant::new_boss(BOSS_HP, BOSS_DAMAGE);
    let mana = find_minimum_mana_to_win(player, boss);
    println!("\nPart 1: it costs at least {} mana to win in easy mode", mana);
    // Part 2
    let player = Combatant::new_player(PLAYER_HP, PLAYER_MANA, Mode::Hard);
    let boss = Combatant::new_boss(BOSS_HP, BOSS_DAMAGE);
    let mana = find_minimum_mana_to_win(player, boss);
    println!("\nPart 2: it costs at least {} mana to win in hard mode", mana);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let player = Combatant::new_player(10, 250, Mode::Easy);
        let boss = Combatant::new_boss(13, 8);
        let mana = find_minimum_mana_to_win(player, boss);
        assert_eq!(mana, 226);
    }

    #[test]
    fn test_part_1() {
        let player = Combatant::new_player(PLAYER_HP, PLAYER_MANA, Mode::Easy);
        let boss = Combatant::new_boss(BOSS_HP, BOSS_DAMAGE);
        let mana = find_minimum_mana_to_win(player, boss);
        assert_eq!(mana, 1824);
    }

    #[test]
    fn test_part_2() {
        let player = Combatant::new_player(PLAYER_HP, PLAYER_MANA, Mode::Hard);
        let boss = Combatant::new_boss(BOSS_HP, BOSS_DAMAGE);
        let mana = find_minimum_mana_to_win(player, boss);
        assert_eq!(mana, 1937);
    }
}
