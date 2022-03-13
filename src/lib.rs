use wasm_bindgen::prelude::wasm_bindgen;

pub mod constants;
pub mod game;
pub mod gui;
pub mod logic;
pub mod model;
pub mod utils;

use game::Game;

#[wasm_bindgen]
pub fn game(canvas_id: &str, width: u32, height: u32, px: u32, fps: u32) {
    let game = Game::new(canvas_id, width, height, px);
    game.play(fps as i32);
}
