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

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Farkle {
    #[serde(skip)]
    players: Vec<Player>,
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

impl Farkle {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    fn settings(&mut self, ui: &mut Ui) -> Option<AppAction> {
        ui.label("Number of turns");
        ui.add(egui::Slider::new(&mut self.turn_count, 1..=20usize));

        ui.label("Number of players");
        ui.add(egui::Slider::new(&mut self.player_count, 0..=10usize));
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

    fn splash(&self, ui: &mut Ui) {}

    fn game_view(&mut self, ui: &mut Ui) {
        ui.label(format!(
            "{}'s turn {} of {}",
            self.players[self.current_player].name(),
            self.current_turn,
            self.turn_count
        ));
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
                            .take(self.player_count)
                            .map(|name| Player::new(name.clone()))
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
                self.game_view(ui)
            } else {
                self.splash(ui);
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Farkle",
        native_options,
        Box::new(|cc| Box::new(Farkle::new(cc))),
    );
}
