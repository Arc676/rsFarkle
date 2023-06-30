// Copyright (C) 2023 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3)

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License

use rsfarkle::farkle::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rsfarkle", about = "Command line Farkle game")]
struct Options {
    #[structopt(short = "p", long = "players", help = "Player count")]
    player_count: usize,
    #[structopt(short = "t", long = "turns", help = "Turn count")]
    turn_count: u32,
}

type PlayerList = Vec<Player>;

fn view_roll(roll: &Roll) {
    println!("Your roll:");
    for i in 1..=6 {
        print!("{} ", i);
    }
    println!("\n------------");
    for die in roll.dice() {
        if die.picked() {
            print!("- ");
        } else {
            print!("{} ", die.value());
        }
    }
}

fn play_game(players: &mut PlayerList, turns: u32) {
    for turn in 1..=turns {
        for player in players.iter() {
            println!(
                "{}'s turn {} of {}. Current score: {}.",
                player.name(),
                turn,
                turns,
                player.score()
            );
        }
    }
}

fn save_scores(players: &mut PlayerList) {
    //
}

fn main() {
    let Options {
        player_count,
        turn_count,
    } = Options::from_args();

    let mut players = PlayerList::with_capacity(player_count);

    for _ in 0..player_count {
        players.push(Player::new("".to_string()));
    }

    play_game(&mut players, turn_count);

    save_scores(&mut players);
}
