#![allow(dead_code, unused_mut, unused_variables)]

extern crate sdl2;
extern crate core_compat;
//extern crate rusqlite;

mod data_manager;
mod entry;
mod error;
mod fps;
mod game;
mod map_manager;
mod sprite_manager;
mod sprite_type;
mod sprite;
mod vec;
mod sdl;
#[cfg(test)]
mod test;

use std::path::Path;

use fps::FpsTimer;
use game::Game;
use sdl::Sdl;
use map_manager::MapManager;
use sprite_manager::SpriteManager;
use sprite_type::SpriteType;
use data_manager::DataManager;
use entry::Entry;

const WINDOW_WIDTH:u32 = 800;
const WINDOW_HEIGHT:u32 = 600;

fn main() {

    // Setup SDL2
    let mut sdl = Sdl::new(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    sdl.init_game_controllers().unwrap();

    // Setup game state
    let mut game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut maps = MapManager::new(&Path::new("../data/DATAs/Map/"));
    let mut sprites = SpriteManager::new(&Path::new("../data/RLEs/"));
    let mut datas = DataManager::new(&Path::new("../data/DATAs/"));

    // inital loop state
    let mut fps_timer = FpsTimer::new(60.0);
    let mut last_sec = 0;

    // experiment
    let entry = Entry { file: 0, index: 3 };
    let interface_t = SpriteType::Interface;
    let sprite = sprites.get_sprite(entry, interface_t).unwrap();
    game.state.sprite = Some(sprite);

    'main: loop {

        // loop start time
        fps_timer.tick();
        let tick = fps_timer.get_epoch().elapsed().as_secs();
        if tick > last_sec {
            let dur = fps_timer.get_frame_time();
            let (sec, ns) = (dur.as_secs(), dur.subsec_nanos() as f32);
            println!("Frame time: {}(s):{}ms", sec, ns / 1_000_000.0);
            last_sec = tick;
        }

        // start event handler
        let (exit, dim) = sdl.handle_events(&mut game);
        if exit { break 'main; }
        if let Some((x, y)) = dim {
            game.resize_buffer(x as u32, y as u32);
        }

        // Update game
        game.update_and_render();

        // render our window
        sdl.draw_buffer_surface(&game.render_buffer.memory).unwrap();

        // start frame timing calculations
        fps_timer.sleep_til_next_tick();

    }
}
