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
mod sdl_state;
mod sprite_manager;
mod sprite_type;
mod sprite;
mod vec;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use fps::FpsTimer;
use game::Game;
use sdl_state::SdlState;

fn main() {

    // Setup game state
    let mut game = Game::new();
    let mut sdl = SdlState::new().unwrap();

    // inital loop state
    let mut fps_timer = FpsTimer::new(60.0);
    let mut last_sec = 0;
    let mut last_event = None;

    'main: loop {
        // loop start time
        fps_timer.tick();
        let tick = fps_timer.get_epoch().elapsed().as_secs();
        if tick > last_sec {
            println!("fps: {:?}", fps_timer.get_last_fps());
            last_sec = tick;
        }

        // start event handler
        let new_event = sdl.event.poll_event();
        if new_event != last_event {
            if let Some(ref event) = new_event {
                match event {
                    &Event::Quit { .. } |
                    &Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main;
                    }
                    _ => {
                        println!("recieved: {:?}", event);
                    }
                }
            }
        }

        if new_event.is_some() {
            last_event = new_event;
        }
        // end of event handler

        // start frame timing calculations
        fps_timer.sleep_til_next_tick();

    }
}
