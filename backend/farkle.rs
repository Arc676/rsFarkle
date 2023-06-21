// Copyright (C) 2023 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3)

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License

use rand::Rng;

const STRAIGHT_VALUE: i32 = 3000;
const TRIPLE_PAIR_VALUE: i32 = 2000;

#[derive(Debug)]
pub enum GameState {
    FirstRoll,
    Rolling,
    Picking,
    TurnEnded,
}

#[derive(Debug)]
pub enum RollType {
    Farkle,
    Simple,
    TriplePair,
    Straight,
}

#[derive(Debug)]
pub enum ToggleResult {
    Picked,
    Unpicked,
    NotPickable,
    NotUnpickable,
}

#[derive(Debug, Default)]
pub struct Die {
    value: i32,
    picked: bool,
    picked_this_roll: bool,
}

#[derive(Debug, Default)]
pub struct Roll {
    dice: [Die; 6],
}

#[derive(Debug, Default)]
pub struct Selection {
    values: [i32; 6],
    die_count: i32,
    value: i32,
}

type Hand = Vec<Selection>;

#[derive(Debug)]
pub struct Player {
    hand: Hand,
    score: i32,
    name: String,
}

impl Die {
    fn pick(&mut self) {
        self.picked = true;
        self.picked_this_roll = true;
    }

    fn unpick(&mut self) {
        self.picked = false;
        self.picked_this_roll = false;
    }
}

impl Roll {
    fn is_exhausted(&self) -> bool {
        for die in &self.dice {
            if !die.picked {
                return false;
            }
        }
        true
    }

    fn count_values(&self) -> [usize; 6] {
        let mut res = [0; 6];
        for die in &self.dice {
            if !die.picked || die.picked_this_roll {
                res[die.value as usize] += 1;
            }
        }
        res
    }

    fn determine_pickable(&self, occurrences: Option<&[usize; 6]>) -> [bool; 6] {
        let mut res = [false; 6];
        let counts = match occurrences {
            Some(c) => *c,
            None => self.count_values(),
        };
        for i in 0..6 {
            let required = if i == 0 || i == 4 { 3 } else { 1 };
            let count = counts[(self.dice[i].value - 1) as usize];
            res[i] = !self.dice[i].picked && count >= required;
        }
        res
    }

    fn pick_die(&mut self, die: usize) -> bool {
        let allowed = self.determine_pickable(None);
        if allowed[die] {
            self.dice[die].pick();
            return true;
        }
        false
    }

    fn unpick_die(&mut self, die: usize) -> bool {
        let die = &mut self.dice[die];
        if die.picked_this_roll {
            die.unpick();
            return true;
        }
        false
    }

    pub fn new_roll(&mut self) {
        if self.is_exhausted() {
            *self = Roll::default();
        }
        for die in &mut self.dice {
            if die.picked {
                die.picked_this_roll = false;
            } else {
                die.value = rand::thread_rng().gen_range(1..=6);
            }
        }
    }

    pub fn toggle_die(&mut self, die: usize) -> ToggleResult {
        if self.dice[die].picked {
            if self.unpick_die(die) {
                ToggleResult::Unpicked
            } else {
                ToggleResult::NotUnpickable
            }
        } else {
            if self.pick_die(die) {
                ToggleResult::Picked
            } else {
                ToggleResult::NotPickable
            }
        }
    }

    pub fn determine_type(&mut self, selection: &mut Selection) -> RollType {
        let counts = self.count_values();

        let mut is_straight = true;
        let mut is_triple_pair = true;

        for c in counts {
            if c != 1 {
                is_straight = false;
            }
            if c != 2 {
                is_triple_pair = false;
            }
            if !is_straight && !is_triple_pair {
                break;
            }
        }

        if is_straight || is_triple_pair {
            for i in 0..6 {
                selection.values[i] = self.dice[i].value;
                self.dice[i].pick();
            }
            selection.die_count = 6;
            if is_straight {
                selection.value = STRAIGHT_VALUE;
                return RollType::Straight;
            } else {
                selection.value = TRIPLE_PAIR_VALUE;
                return RollType::TriplePair;
            }
        }

        let pickable = self.determine_pickable(Some(&counts));
        for allowed in pickable {
            if allowed {
                return RollType::Simple;
            }
        }
        RollType::Farkle
    }
}
