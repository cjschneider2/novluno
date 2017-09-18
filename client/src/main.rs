#![allow(dead_code)]
extern crate gl;
extern crate sdl2;
extern crate rusttype;
extern crate core_compat;

mod error;
mod game;
mod sdl;

use sdl::Sdl;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;


fn main() {

    // setup Rusttype
    let font_data = include_bytes!("../static/noto_font/NotoMono-Regular.ttf");
    let font_collection = rusttype::FontCollection::from_bytes(font_data as &[u8]);

    // Setup SDL2
    let mut sdl = Sdl::new(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    sdl.init_game_controllers().unwrap();

    'main: loop {
        // start event handler

        // Update game

        // render our window

        // DEBUG
        std::thread::sleep_ms(10000);

        break 'main;
    }
}
