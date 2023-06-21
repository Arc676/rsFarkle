// Copyright (C) 2023 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3)

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License

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

#[derive(Debug)]
pub struct Die {
    value: i32,
    picked: bool,
    picked_this_roll: bool,
}

#[derive(Debug)]
pub struct Roll {
    dice: [Die; 6],
}

#[derive(Debug)]
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
