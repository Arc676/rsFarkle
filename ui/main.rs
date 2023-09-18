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

// Based on code in https://github.com/emilk/eframe_template

use eframe::egui::{Context, Ui};
use eframe::{egui, Frame};

use rsfarkle::farkle::*;

#[forbid(unsafe_code)]
#[derive(Debug, PartialEq)]
enum AppAction {
    StartGame,
    ExitApp,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Farkle {
    #[serde(skip)]
    players: Vec<Player>,
    #[serde(skip)]
    roll: Roll,
    #[serde(skip)]
    state: GameState,
    #[serde(skip)]
    roll_state: Option<RollType>,

    #[serde(skip)]
    current_turn: usize,
    #[serde(skip)]
    current_player: usize,
    #[serde(skip)]
    game_in_progress: bool,

    player_names: Vec<String>,
    player_count: usize,
    turn_count: usize,
}

impl Default for Farkle {
    fn default() -> Self {
        Farkle {
            players: vec![],
            current_turn: 0,
            current_player: 0,
            game_in_progress: false,
            player_names: vec![],
            player_count: 1,
            turn_count: 5,
            roll: Roll::default(),
            state: GameState::default(),
            roll_state: None,
        }
    }
}

impl Farkle {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    fn get_current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    fn get_current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    fn get_input(name: &str, key: egui::Key, ctx: &Context, ui: &mut Ui) -> bool {
        ui.button(name).clicked() || ctx.input(|i| i.key_released(key))
    }

    fn settings(&mut self, ui: &mut Ui) -> Option<AppAction> {
        ui.label("Number of turns");
        ui.add(egui::Slider::new(&mut self.turn_count, 1..=20usize));

        ui.label("Number of players");
        ui.add(egui::Slider::new(&mut self.player_count, 1..=10usize));
        if self.player_count > self.player_names.len() {
            self.player_names
                .resize_with(self.player_count, || String::new());
        }
        for name in self.player_names.iter_mut().take(self.player_count) {
            ui.text_edit_singleline(name);
        }
        if ui.button("New Game").clicked() {
            return Some(AppAction::StartGame);
        }
        if ui.button("Quit").clicked() {
            return Some(AppAction::ExitApp);
        }
        None
    }

    fn splash(&self, ui: &mut Ui) {
        ui.heading("Farkle");
        ui.label("Set up game parameters and click 'New Game' to play.");
    }

    fn game_view(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.label(format!(
            "{}'s turn {} of {}. Score: {}",
            self.get_current_player().name(),
            self.current_turn,
            self.turn_count,
            self.get_current_player().score()
        ));

        if let Some(roll) = self.roll_state {
            match roll {
                RollType::Farkle => ui.label("Farkle!"),
                RollType::TriplePair => ui.label("Triple pair!"),
                RollType::Straight => ui.label("Straight!"),
                _ => ui.label("")
            };
        }

        let mut mov = None;

        type Mapping = (&'static str, egui::Key, MoveType);
        const MOVES: [Mapping; 3] = [
            ("Roll", egui::Key::R, MoveType::Roll),
            ("Confirm Selection", egui::Key::C, MoveType::Pick),
            ("Bank", egui::Key::B, MoveType::Bank),
        ];

        ui.horizontal(|ui| {
            for (name, key, mt) in MOVES {
                if Self::get_input(name, key, ctx, ui) {
                    mov = Some(mt);
                }
            }
        });

        let Some(mov) = mov else { return };
        match mov {
            MoveType::Roll => {
                self.roll.new_roll();
                let (selection, roll_type) = self.roll.determine_type();
                match roll_type {
                    RollType::Farkle => {
                        self.get_current_player_mut().empty_hand();
                        self.state = GameState::TurnEnded;
                        self.roll_state = Some(roll_type);
                    }
                    RollType::Straight | RollType::TriplePair => {
                        self.get_current_player_mut().add_selection(selection);
                        self.roll_state = Some(roll_type);
                    }
                    _ => self.state = GameState::Picking,
                }
            }
            MoveType::Bank => {
                self.get_current_player_mut().bank();
                self.state = GameState::TurnEnded;
            }
            _ => todo!(),
        }
    }
}

impl eframe::App for Farkle {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            if let Some(action) = self.settings(ui) {
                match action {
                    AppAction::StartGame => {
                        self.players = self
                            .player_names
                            .iter()
                            .enumerate()
                            .take(self.player_count)
                            .map(|(i, name)| {
                                if name.is_empty() {
                                    Player::new(format!("Player {}", i + 1))
                                } else {
                                    Player::new(name.clone())
                                }
                            })
                            .collect();

                        self.current_turn = 1;
                        self.current_player = 0;
                        self.game_in_progress = true;
                    }
                    AppAction::ExitApp => frame.close(),
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.game_in_progress {
                self.game_view(ctx, ui)
            } else {
                self.splash(ui);
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Farkle",
        native_options,
        Box::new(|cc| Box::new(Farkle::new(cc))),
    )
}
