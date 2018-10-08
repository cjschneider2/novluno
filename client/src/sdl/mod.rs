extern crate geometry;

mod render;
mod controller;

use std::cell::RefCell;

use sdl2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;

use rusttype;

use crate::error::Error;
use crate::game::Game;
use crate::game::input::Controller;
use crate::game::input::MAX_CONTROLLERS as MAX_CTL;

// setup Rusttype
lazy_static! {
    static ref FONT: rusttype::Font<'static> = {
        let font_data = include_bytes!("../../static/noto_font/NotoMono-Regular.ttf");
        let font_collection = rusttype::FontCollection::from_bytes(font_data as &[u8]).expect("cannot create font collection");
        font_collection.into_font().unwrap()
    };
}

pub struct Sdl {
    pub context: sdl2::Sdl,
    // video/rendering
    pub video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    // audio
    pub audio: sdl2::AudioSubsystem,
    pub audio_spec: sdl2::audio::AudioSpecDesired,
    // event handlers
    pub event_pump: RefCell<sdl2::EventPump>,
    // controllers
    pub controller: sdl2::GameControllerSubsystem,
    pub controllers: RefCell<[Option<sdl2::controller::GameController>; MAX_CTL]>,
    pub controller_count: u32,
    // debug
    pub do_debug_output: bool,
}

impl Sdl {
    pub fn new(width: u32, height: u32) -> Result<Sdl, Error> {
        // -- load SDL2 contexts
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video.window("Novluno", width, height)
            .position_centered()
            .opengl()
            .resizable()
            .build()?;
        let canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build()?;
        let texture_creator = canvas.texture_creator();
        let controller = context.game_controller()?;
        let controllers = RefCell::new([None, None, None, None]);
        let event_pump = RefCell::new(context.event_pump()?);
        let audio = context.audio()?;
        let audio_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(2),
            samples: Some(4),
        };

        // -- Create SDL state object
        let sdl = Sdl {
            context,
            video,
            canvas,
            texture_creator,
            event_pump,
            audio,
            audio_spec,
            controller,
            controllers,
            controller_count: 0,
            do_debug_output: true,
        };
        Ok(sdl)
    }

    pub fn handle_events(
        &mut self,
        game: &mut Game,
    ) {
        let mut event_pump = self.event_pump.borrow_mut();
        let mut last_event: Option<Event> = None;
        while let Some(new_event) = event_pump.poll_event() {
            if last_event.is_none() || new_event != last_event.unwrap() {
                match new_event {
                    Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                    => {
                        game.input.should_quit = true;
                    }
                    Event::KeyDown { keycode: Some(key), repeat, .. }
                    => {
                        let is_down = true;
                        if !repeat {
                            process_keycode(key, is_down, game.get_mut_keyboard());
                        }
                    }
                    Event::KeyUp { keycode: Some(key), repeat, .. }
                    => {
                        let is_down = false;
                        if !repeat {
                            process_keycode(key, is_down, game.get_mut_keyboard());
                        }
                    }
                    Event::Window { win_event: w_event, .. } => {
                        match w_event {
                            WindowEvent::Enter => (),
                            WindowEvent::Leave => (),
                            WindowEvent::SizeChanged(x, y) => {
                                game.input.should_resize = Some((x, y));
                                println!("Window size change: ({},{})", x, y);
                            }
                            _ => (),
                        }
                    }
                    Event::MouseMotion { x, y, .. } => {
                        game.input.mouse_x = x;
                        game.input.mouse_y = y;
                    }
                    Event::MouseButtonDown { mouse_btn: btn, .. } => {
                        let is_down = true;
                        match btn {
                            sdl2::mouse::MouseButton::Left => {
                                game.input.mouse_left.key_press(is_down);
                            }
                            sdl2::mouse::MouseButton::Middle => {
                                game.input.mouse_middle.key_press(is_down);
                            }
                            sdl2::mouse::MouseButton::Right => {
                                game.input.mouse_right.key_press(is_down);
                            }
                            _ => {}
                        }
                    }
                    Event::MouseButtonUp { mouse_btn: btn, .. } => {
                        let is_down = false;
                        match btn {
                            sdl2::mouse::MouseButton::Left => {
                                game.input.mouse_left.key_press(is_down);
                            }
                            sdl2::mouse::MouseButton::Middle => {
                                game.input.mouse_middle.key_press(is_down);
                            }
                            sdl2::mouse::MouseButton::Right => {
                                game.input.mouse_right.key_press(is_down);
                            }
                            _ => {}
                        }
                    }
                    Event::ControllerDeviceAdded { which: index, .. } => {
                        println!("{:?}: {:?}", new_event, index);
                        controller::add(self, index).unwrap();
                    }
                    Event::ControllerDeviceRemoved { which: index, .. } => {
                        println!("{:?}: {:?}", new_event, index);
                        controller::remove(self, index).unwrap();
                    }
                    Event::JoyDeviceAdded { .. } => (),
                    _ => {
                        println!("{:?}", new_event);
                    }
                }
            }
            last_event = Some(new_event);
        } // end while new SDL event
    }

    pub fn render(&mut self, game: &mut Game, _dt: f32) {
        // start frame
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(75, 100, 255));

        // draw background color
        self.canvas.clear();

        // render
        // -- game map
        {
            render::map::tiles(self, game);
            render::map::objects(self, game);
        }
        // -- character(s) / NPC(s)
        {
            render::chars::chars(self, game);
        }
        // -- skill(s)
        // -- window(s)
        // -- interface(s)
        // -- window-chrome

        // draw text
        {
            // let text = format!("{}, {}", game.input.mouse_x, game.input.mouse_y);
            // self.render_text_line(&text, game.input.mouse_x, game.input.mouse_y);
        }

        // finish frame
        self.canvas.present();
    }
}

fn process_keycode(
    key: sdl2::keyboard::Keycode,
    is_down: bool,
    input: &mut Controller,
) {
    match key {
        Keycode::W     => input.move_up.key_press(is_down),
        Keycode::A     => input.move_left.key_press(is_down),
        Keycode::S     => input.move_down.key_press(is_down),
        Keycode::D     => input.move_right.key_press(is_down),
        Keycode::Q     => input.left_shoulder.key_press(is_down),
        Keycode::E     => input.right_shoulder.key_press(is_down),
        Keycode::Up    => input.action_up.key_press(is_down),
        Keycode::Down  => input.action_down.key_press(is_down),
        Keycode::Right => input.action_right.key_press(is_down),
        Keycode::Left  => input.action_left.key_press(is_down),
        Keycode::K     => input.player_up.key_press(is_down),
        Keycode::J     => input.player_down.key_press(is_down),
        Keycode::H     => input.player_right.key_press(is_down),
        Keycode::L     => input.player_left.key_press(is_down),
        Keycode::F     => (),
        Keycode::Space => (),
        _              => (),
    }
}
