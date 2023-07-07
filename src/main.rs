// Copyright (C) 2023 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3)

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License

use chrono::Local;
use std::{
    fs::File,
    io::{self, Write},
};

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

enum MoveType {
    Roll,
    Bank,
    Exit,
    View,
    Pick,
    Help,
    Hand,
    Unpick,
}

fn print_help() {
    println!(concat!(
        "help - show this help text\n",
        "roll - roll die pool\n",
        "view - view the current roll\n",
        "pick - pick dice from the die pool\n",
        "unpick - reset the die selection\n",
        "hand - show your current hand\n",
        "bank - bank all points currently in hand\n",
        "exit - immediately exit the game"
    ))
}

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

fn get_move() -> Option<MoveType> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    match input.trim() {
        "help" => Some(MoveType::Help),
        "roll" => Some(MoveType::Roll),
        "bank" => Some(MoveType::Bank),
        "exit" => Some(MoveType::Exit),
        "view" => Some(MoveType::View),
        "pick" => Some(MoveType::Pick),
        "hand" => Some(MoveType::Hand),
        "unpick" => Some(MoveType::Unpick),
        _ => None,
    }
}

fn play_game(players: &mut PlayerList, turns: u32) {
    let mut roll = Roll::default();
    'game_loop: for turn in 1..=turns {
        for (player_no, player) in players.iter_mut().enumerate() {
            println!(
                "{}'s turn {} of {}. Current score: {}.",
                player.name(),
                turn,
                turns,
                player.score()
            );

            roll.new_roll();
            let mut state = GameState::FirstRoll;

            while state != GameState::TurnEnded {
                print!("{}> ", player_no);
                let cmd = get_move();
                if cmd.is_none() {
                    println!("Invalid command. Type 'help' to see a list of commands.");
                    continue;
                }
                let cmd = cmd.unwrap();
                match cmd {
                    MoveType::Roll => {
                        if state == GameState::Picking {
                            println!("You have already rolled. Use 'pick' to pick from the die pool.");
                            continue;
                        }
                        roll.new_roll();
                        view_roll(&roll);

                        let (selection, roll_type) = roll.determine_type();
                        match roll_type {
                            RollType::Farkle => {
                                println!("Farkle!");
                                player.empty_hand();
                                state = GameState::TurnEnded;
                            }
                            RollType::Straight | RollType::TriplePair => {
                                println!("{}!\nSelected {} points' worth of dice.", roll_type, selection.value());
                                player.add_selection(selection);
                            }
                            _ => state = GameState::Picking,
                        }
                    }
                    MoveType::Bank => {
                        if state == GameState::Rolling {
                            let points = player.bank();
                            println!("Banked {} points.", points);
                            state = GameState::TurnEnded;
                        } else {
                            println!("You must pick from the die pool before banking.");
                        }
                    }
                    MoveType::Exit => break 'game_loop,
                    MoveType::View => view_roll(&roll),
                    MoveType::Pick => todo!(),
                    MoveType::Help => print_help(),
                    MoveType::Hand => {
                        let mut total = 0;
                        println!("Your selections:");
                        for sel in player.selections() {
                            for value in sel.values() {
                                print!("{} ", value);
                            }
                            println!();
                            total += sel.value();
                        }
                        println!("{} points in hand.", total);
                    }
                    MoveType::Unpick => todo!(),
                }
            }
        }
    }
    println!("Game over");
}

fn save_scores(players: &mut PlayerList) -> io::Result<()> {
    print!("Enter filename for scores: ");
    io::stdout().flush()?;
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    filename = filename.trim().to_string();

    players.sort();
    let now = Local::now();

    if filename.is_empty() {
        println!("{}", now.format("%F %T"));
        for player in players {
            println!("{} - {}", player.name(), player.score());
        }
    } else {
        let mut file = File::create(&filename)?;
        write!(file, "{}", now.format("%F %T"))?;
        for player in players {
            write!(file, "{} - {}", player.name(), player.score())?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let Options {
        player_count,
        turn_count,
    } = Options::from_args();

    let mut players = PlayerList::with_capacity(player_count);

    for i in 0..player_count {
        print!("Enter name for player {}: ", i + 1);
        io::stdout().flush()?;
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;
        players.push(Player::new(name.trim().to_string()));
    }

    play_game(&mut players, turn_count);

    save_scores(&mut players)?;

    Ok(())
}
