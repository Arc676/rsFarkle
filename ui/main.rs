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
#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Farkle {
    #[serde(skip)]
    players: Vec<Player>,
    #[serde(skip)]
    current_turn: usize,
    #[serde(skip)]
    current_player: usize,

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

    fn settings(&mut self, ui: &mut Ui) -> bool {
        ui.button("Quit").clicked()
    }

    fn game_view(&mut self, ui: &mut Ui) {
        //
    }
}

impl eframe::App for Farkle {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            if self.settings(ui) {
                frame.close();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| self.game_view(ui));
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
