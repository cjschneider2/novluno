
// mod audio;

use std::cell::RefCell;
use std::borrow::BorrowMut;

use sdl2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;

use error::Error;
use game::input::Controller;
use game::input::MAX_CONTROLLERS as MAX_CTL;

pub struct Sdl {
    pub context: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub audio: sdl2::AudioSubsystem,
    pub audio_spec: sdl2::audio::AudioSpecDesired,
    pub event_pump: RefCell<sdl2::EventPump>,
    pub last_event: RefCell<Option<sdl2::event::Event>>,
    pub controller: sdl2::GameControllerSubsystem,
    pub controllers: RefCell<[Option<sdl2::controller::GameController>; MAX_CTL]>,
    pub controller_count: u32,
}

impl Sdl {
    pub fn new(width: u32, height: u32) -> Result<Sdl, Error> {
        let context = sdl2::init()?;
        let video = context.video()?;
        // TODO: The window should eventually be able to be resized, but this requires
        //       changing the backing renderer to use Textures and not 
        let window = video.window("Novluno", width, height)
                          .position_centered()
                          // .resizable() 
                          .opengl()
                          .build()?;
        let controller = context.game_controller()?;
        let controllers = RefCell::new([None, None, None, None]);
        let event_pump = RefCell::new(context.event_pump()?);
        let audio = context.audio()?;
        let audio_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(2),
            samples: Some(4),
        };
        let sdl = Sdl {
            context: context,
            video: video,
            window: window,
            event_pump: event_pump,
            audio: audio,
            last_event: RefCell::new(None),
            audio_spec: audio_spec,
            controller: controller,
            controllers: controllers,
            controller_count: 0,
        };
        Ok(sdl)
    }

    pub fn init_game_controllers(&mut self) -> Result<(), Error> {
        let num_joy = self.controller.num_joysticks()?;
        if self.controller_count != num_joy {
            let max = MAX_CTL as u32;
            let max = if num_joy < max { num_joy } else { max };
            for index in 1..max {
                println!("Found Controller index: {:?}", index);
            }
            self.controller_count = num_joy;
        }
        Ok(())
    }

    pub fn add_game_controller(&self, index: i32) -> Result<(), Error> {
        let mut controllers = self.controllers.borrow_mut();
        if index < MAX_CTL as i32 && index > 0 {
            let ctrl_list = controllers.borrow_mut();
            let new_ctrl = self.controller.open(index as u32)?;
            if let Some(spot) = ctrl_list.get_mut(index as usize) {
                *spot = Some(new_ctrl);
            }
        }
        println!("added controller: {}", index);
        Ok(())
    }

    pub fn remove_game_controller(&self, index: i32) -> Result<(), Error> {
        let mut controllers = self.controllers.borrow_mut();
        if index < MAX_CTL as i32 && index > 0 {
            let ctrl_list = controllers.borrow_mut();
            if let Some(spot) = ctrl_list.get_mut(index as usize) {
                *spot = None;
            }
        }
        println!("removed controller: {}", index);
        Ok(())
    }

    pub fn handle_events(
        &self,
        game: &mut ::game::Game
    ) -> (bool, Option<(i32, i32)>) {
        let mut should_quit = false;
        let mut event_pump = self.event_pump.borrow_mut();
        let mut last_event = self.last_event.borrow_mut();
        let mut resize = None;
        let new_event = event_pump.poll_event();
        if new_event != *last_event {
            if let Some(ref event) = new_event {
                match event {
                    &Event::Quit { .. }
                    | &Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                    => {
                        should_quit = true;
                    },
                    &Event::KeyDown { keycode: Some(key), repeat, .. }
                    => {
                        let is_down = true;
                        if !repeat {
                            process_keycode(key, is_down, game.get_mut_keyboard());
                        }
                    },
                    &Event::KeyUp { keycode: Some(key), repeat, .. }
                    => {
                        let is_down = false;
                        if !repeat {
                            process_keycode(key, is_down, game.get_mut_keyboard());
                        }
                    },
                    &Event::Window { win_event: w_event, ..} => {
                        match w_event {
                            WindowEvent::Enter => (),
                            WindowEvent::Leave => (),
                            WindowEvent::SizeChanged(x, y) => {
                                resize = Some ((x, y));
                                println!("Window size change: ({},{})", x, y);
                            },
                            _ => (),
                        }
                    },
                    &Event::MouseMotion { .. } => (),
                    &Event::ControllerDeviceAdded { which: index, .. } => {
                        println!("{:?}: {:?}", event, index);
                        self.add_game_controller(index).unwrap();
                    },
                    &Event::ControllerDeviceRemoved { which: index, .. } => {
                        println!("{:?}: {:?}", event, index);
                        self.remove_game_controller(index).unwrap();
                    },
                    &Event::JoyDeviceAdded { .. } => (),
                    _ => {
                        println!("{:?}", event);
                    },
                }
            }
        }
        if new_event.is_some() {
            *last_event = new_event;
        }
        (should_quit, resize)
    }

    pub fn draw_buffer_surface<'a> (
        &mut self,
        buffer: &[u8],
    ) -> Result<(), Error> {
        let event_pump = &self.event_pump.borrow();
        let mut surface = self.window.surface(event_pump)?;
        surface.with_lock_mut(|buf:&mut [u8]| {
            if buf.len() == buffer.len() {
                buf.copy_from_slice(buffer);
            }
        });
        surface.update_window()?;
        Ok(())
    }

}

fn process_keycode(
    key: sdl2::keyboard::Keycode,
    is_down: bool,
    input: &mut Controller
) {
    match key {
        Keycode::W => input.move_up.key_press(is_down),
        Keycode::A => input.move_left.key_press(is_down),
        Keycode::S => input.move_down.key_press(is_down),
        Keycode::D => input.move_right.key_press(is_down),
        Keycode::Q => input.left_shoulder.key_press(is_down),
        Keycode::E => input.right_shoulder.key_press(is_down),
        Keycode::Up => input.action_up.key_press(is_down),
        Keycode::Down => input.action_down.key_press(is_down),
        Keycode::Right => input.action_right.key_press(is_down),
        Keycode::Left => input.action_left.key_press(is_down),
        Keycode::F => (),
        Keycode::Space => (),
        _ => (),
    }
}
