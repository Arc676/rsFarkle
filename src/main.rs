// Copyright (C) 2023 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3)

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use chrono::Local;
#[cfg(feature = "onekey")]
use std::io::Read;
use std::{
    fs::File,
    io::{self, Write},
};

use rsfarkle::farkle::*;

use structopt::StructOpt;
use termios::{tcsetattr, Termios, ICANON, TCSANOW};

#[derive(Debug, StructOpt)]
#[structopt(name = "rsfarkle", about = "Command line Farkle game")]
struct Options {
    #[structopt(short = "p", long = "players", help = "Player count")]
    player_count: usize,
    #[structopt(short = "t", long = "turns", help = "Turn count")]
    turn_count: u32,
}

type PlayerList = Vec<Player>;

#[derive(Debug, PartialEq)]
enum SelectedMove {
    Move(MoveType),
    Exit,
    NoMove,
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
    println!();
}

#[cfg(not(feature = "onekey"))]
fn get_move(player_no: usize) -> SelectedMove {
    print!("{}> ", player_no);
    io::stdout().flush().expect("Failed to flush");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    match input.trim() {
        "help" => SelectedMove::Move(MoveType::Help),
        "roll" => SelectedMove::Move(MoveType::Roll),
        "bank" => SelectedMove::Move(MoveType::Bank),
        "exit" => SelectedMove::Exit,
        "view" => SelectedMove::Move(MoveType::View),
        "pick" => SelectedMove::Move(MoveType::Pick),
        "hand" => SelectedMove::Move(MoveType::Hand),
        "unpick" => SelectedMove::Move(MoveType::Unpick),
        _ => SelectedMove::NoMove,
    }
}

#[cfg(feature = "onekey")]
fn get_move(player_no: usize) -> SelectedMove {
    print!("{}> ", player_no);
    io::stdout().flush().expect("Failed to flush");
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();
    println!();
    match buffer[0] as char {
        '?' => SelectedMove::Move(MoveType::Help),
        'r' => SelectedMove::Move(MoveType::Roll),
        'b' => SelectedMove::Move(MoveType::Bank),
        'e' => SelectedMove::Exit,
        'v' => SelectedMove::Move(MoveType::View),
        'p' => SelectedMove::Move(MoveType::Pick),
        'h' => SelectedMove::Move(MoveType::Hand),
        'u' => SelectedMove::Move(MoveType::Unpick),
        _ => SelectedMove::NoMove,
    }
}

#[cfg(not(feature = "onekey"))]
fn get_pick() -> Option<usize> {
    print!("Picking> ");
    io::stdout().flush().expect("Failed to flush");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    match input.trim().parse() {
        Ok(val) => {
            if 0 < val && val <= 6 {
                Some(val)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[cfg(feature = "onekey")]
fn get_pick() -> Option<usize> {
    print!("Picking> ");
    io::stdout().flush().expect("Failed to flush");
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();
    println!();
    match buffer[0] as char {
        'q' => Some(1),
        'w' => Some(2),
        'e' => Some(3),
        'r' => Some(4),
        't' => Some(5),
        'y' => Some(6),
        _ => None,
    }
}

fn play_game(players: &mut PlayerList, turns: u32) {
    'game_loop: for turn in 1..=turns {
        for (player_no, player) in players.iter_mut().enumerate() {
            println!(
                "{}'s turn {} of {}. Current score: {}.",
                player.name(),
                turn,
                turns,
                player.score()
            );

            let mut roll = Roll::default();
            let mut state = GameState::FirstRoll;

            while state != GameState::TurnEnded {
                match get_move(player_no) {
                    SelectedMove::Move(mov) => match mov {
                        MoveType::Roll => {
                            if state == GameState::Picking {
                                println!(
                                "You have already rolled. Use 'pick' to pick from the die pool."
                            );
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
                                    println!(
                                        "{}!\nSelected {} points' worth of dice.",
                                        roll_type,
                                        selection.value()
                                    );
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
                        MoveType::Pick => match state {
                            GameState::Rolling => println!(
                            "You have already picked dice. Use 'unpick' to reset your selection."
                        ),
                            GameState::FirstRoll => {
                                println!("You have not rolled yet. Use 'roll' to roll.")
                            }
                            _ => {
                                println!("Enter a die index to toggle selecting. Any invalid input to stop picking.");
                                while let Some(idx) = get_pick() {
                                    match roll.toggle_die(idx - 1) {
                                        ToggleResult::Picked => println!("Picked die {}.", idx),
                                        ToggleResult::Unpicked => println!("Unpicked die {}.", idx),
                                        ToggleResult::NotPickable => {
                                            println!("You cannot pick this die.")
                                        }
                                        ToggleResult::NotUnpickable => {
                                            println!("You cannot unpick this die.")
                                        }
                                    }
                                }
                                match roll.construct_selection() {
                                    Ok(selection) => {
                                        println!(
                                            "Selected {} points' worth of dice.",
                                            selection.value()
                                        );
                                        state = GameState::Rolling;
                                        player.add_selection(selection);
                                    }
                                    Err(e) => {
                                        println!("The selection is invalid: {}", e);
                                        roll.deselect();
                                    }
                                }
                            }
                        },
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
                        MoveType::Unpick => {
                            if state != GameState::Rolling {
                                println!("Cannot unpick dice at this time.");
                                continue;
                            }
                            roll.deselect();
                            let _ = player.undo_selection();
                            state = GameState::Picking;
                            println!("Reset die selection.");
                            view_roll(&roll);
                        }
                    },
                    SelectedMove::Exit => break 'game_loop,
                    SelectedMove::NoMove => {
                        println!("Invalid command. Type 'help' to see a list of commands.")
                    }
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
        writeln!(file, "{}", now.format("%F %T"))?;
        for player in players {
            writeln!(file, "{} - {}", player.name(), player.score())?;
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

    let stdin = 0;
    let old = Termios::from_fd(stdin).unwrap();
    let mut new = old.clone();

    if cfg!(feature = "onekey") {
        new.c_lflag &= !ICANON;
        tcsetattr(stdin, TCSANOW, &new).unwrap();
    }

    play_game(&mut players, turn_count);

    if cfg!(feature = "onekey") {
        tcsetattr(stdin, TCSANOW, &old).unwrap();
    }

    save_scores(&mut players)?;

    Ok(())
}
