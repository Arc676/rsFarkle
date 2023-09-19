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

use eframe::egui::{Ui, Vec2};
use eframe::epaint::{ColorImage, TextureHandle};
use rsfarkle::farkle::DieValue;

#[derive(Default)]
pub struct DieRenderer {
    dice: [Option<(TextureHandle, Vec2)>; 6],
}

macro_rules! get_die_sprites {
    ($dice:expr, $ui:expr, $( $idx:expr ),*) => {
        $(
        let sprite = include_bytes!(concat!("dice/", $idx, ".png"));
        let sprite = load_image_from_bytes(sprite, format!("Die {}", $idx), $ui);
        $dice[$idx - 1] = Some(sprite);
        )*
    };
}

// Based on code from
// https://github.com/emilk/egui/blob/0.16.0/eframe/examples/image.rs
pub fn load_image_from_bytes(
    image_data: &[u8],
    name: String,
    ui: &mut Ui,
) -> (TextureHandle, Vec2) {
    let image = image::load_from_memory(image_data).expect("Failed to load image");
    let image_buffer = image.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image_buffer.into_vec();
    let image = ColorImage::from_rgba_unmultiplied(size, &pixels);

    // Allocate a texture:
    let texture = ui.ctx().load_texture(name, image, Default::default());
    let size = Vec2::new(size[0] as f32, size[1] as f32);
    (texture, size)
}

impl DieRenderer {
    pub fn init(&mut self, ui: &mut Ui) -> Result<(), image::ImageError> {
        get_die_sprites!(self.dice, ui, 1, 2, 3, 4, 5, 6);
        Ok(())
    }

    pub fn needs_init(&self) -> bool {
        self.dice[0].is_none()
    }

    pub fn draw_die(&self, value: DieValue, ui: &mut Ui) {
        assert!(value > 0 && value <= 6);
        if let Some((texture, size)) = &self.dice[(value - 1) as usize] {
            ui.image(texture, *size);
        }
    }
}
