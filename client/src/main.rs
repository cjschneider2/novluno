#![allow(dead_code)]
// extern crate rusttype;
extern crate sdl2;
#[macro_use]
extern crate glium;
extern crate imgui;
extern crate glium_sdl2;
extern crate imgui_glium_renderer;

extern crate core_compat;

mod error;
mod game;
mod sdl;

use sdl::Sdl;

use std::time::Instant;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    // setup Rusttype
    // let font_data = include_bytes!("../static/noto_font/NotoMono-Regular.ttf");
    // let font_collection = rusttype::FontCollection::from_bytes(font_data as &[u8]);

    // Setup SDL2
    let mut sdl = Sdl::new(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    sdl.init_game_controllers().unwrap();

    // Setup initial game state
    let mut game = game::Game::new();

    let map_number = 1;
    game.load_map(map_number).unwrap();
    println!("loaded map: {}", map_number);

    'main: loop {
        // start frame
        let start_time = Instant::now();

        // Processes events
        sdl.handle_events(&mut game);

        // Update game
        if game.input.should_quit {
            break 'main;
        }
        game.update();

        // render our window
        let ft = frame_time(&start_time);
        sdl.render(&game, ft);
    }
}

fn frame_time(start: &Instant) -> f32 {
    let elapsed = start.elapsed();
    let sec_ms = (elapsed.as_secs() * 1000) as f32;
    let nano_ms = (elapsed.subsec_nanos() as f32) / 1_000_000.0;
    sec_ms + nano_ms
}
