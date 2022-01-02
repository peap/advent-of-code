use std::cmp;
use std::collections::VecDeque;

use common::{default_puzzle, Puzzle};

pub const BOSS_HP: i64 = 71;
pub const BOSS_DAMAGE: i64 = 10;
pub const PLAYER_HP: i64 = 50;
pub const PLAYER_MANA: u64 = 500;

const MANA_MAGIC_MISSILE: u64 = 53;
const MANA_DRAIN: u64 = 73;
const MANA_SHIELD: u64 = 113;
const MANA_POISON: u64 = 173;
const MANA_RECHARGE: u64 = 229;

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
    hp: i64,
    damage: i64,
    armor: i64,
    mana: u64,
    expenses: u64,
    spell: Spell,
    mode: Mode,
    t_shielded: i64,
    t_poisoned: i64,
    t_recharing: i64,
}

impl Combatant {
    fn new(hp: i64, damage: i64, mana: u64, spell: Spell, mode: Mode) -> Combatant {
        Combatant {
            hp,
            damage,
            armor: 0,
            mana,
            expenses: 0,
            spell,
            mode,
            t_shielded: 0,
            t_poisoned: 0,
            t_recharing: 0,
        }
    }

    fn new_player(hp: i64, mana: u64, mode: Mode) -> Combatant {
        Combatant::new(hp, 0, mana, Spell::Nothing, mode)
    }

    fn new_boss(hp: i64, damage: i64) -> Combatant {
        Combatant::new(hp, damage, 0, Spell::BossAttack, Mode::Easy)
    }

    fn buy(&mut self, mana: u64) {
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
            self.mana += 101; // doesn't subtract from expenses
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

pub fn find_minimum_mana_to_win(player: Combatant, boss: Combatant) -> u64 {
    let mut minimum_mana = u64::max_value();
    let mut q: VecDeque<(Combatant, Combatant)> = VecDeque::new();
    for p in player.get_next_spells(&boss).iter() {
        q.push_back((p.clone(), boss.clone()));
    }
    while !q.is_empty() {
        let (ref mut player, ref mut boss) = q.pop_front().unwrap();
        // Player turn
        player.apply_effects(true);
        if player.dead() {
            continue;
        }
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
        if player.dead() {
            continue;
        }
        // Get next moves
        for p in player.get_next_spells(boss) {
            q.push_back((p.clone(), boss.clone()));
        }
    }
    minimum_mana
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Wizard Simulator 20XX");
    puzzle.set_part1("least mana to win (easy)", |_| {
        let player = Combatant::new_player(PLAYER_HP, PLAYER_MANA, Mode::Easy);
        let boss = Combatant::new_boss(BOSS_HP, BOSS_DAMAGE);
        find_minimum_mana_to_win(player, boss)
    });
    puzzle.set_part2("least mana to win (hard)", |_| {
        let player = Combatant::new_player(PLAYER_HP, PLAYER_MANA, Mode::Hard);
        let boss = Combatant::new_boss(BOSS_HP, BOSS_DAMAGE);
        find_minimum_mana_to_win(player, boss)
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
    fn test_example_1() {
        let player = Combatant::new_player(10, 250, Mode::Easy);
        let boss = Combatant::new_boss(13, 8);
        let mana = find_minimum_mana_to_win(player, boss);
        assert_eq!(mana, 226);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(1824);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(1937);
    }
}
