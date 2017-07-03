#![allow(dead_code)]

extern crate sdl2;
extern crate core_compat;

mod error;
mod fps;
mod sprite;
mod sprite_type;
mod sprite_manager;
mod entry;
mod vec;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use fps::FpsTimer;

fn main() {

   let sdl_context = sdl2::init().unwrap();
   let video_subsystem = sdl_context.video().unwrap();

   let window = video_subsystem.window("rs_hero", 400, 400 )
                               .position_centered()
                               .opengl()
                               .build()
                               .unwrap();

   let mut _canvas = window.into_canvas().present_vsync().build().unwrap();
   let mut event_pump = sdl_context.event_pump().unwrap();

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
      let new_event = event_pump.poll_event();
      if new_event != last_event {
         if let Some(ref event) = new_event {
            match event {
                 &Event::Quit { .. }
               | &Event::KeyDown { keycode: Some(Keycode::Escape), .. }
               => {
                  break 'main;
               },
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
