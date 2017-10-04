pub mod gui;

use std::cell::RefCell;
use std::borrow::BorrowMut;

use imgui;

use imgui_glium_renderer;

use glium;
use glium::Surface;
use glium::index::PrimitiveType;

use glium_sdl2;
use glium_sdl2::DisplayBuild;

use sdl2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;

use error::Error;
use game::Game;
use game::input::Controller;
use game::input::MAX_CONTROLLERS as MAX_CTL;

pub struct Sdl {
    pub context: sdl2::Sdl,
    // video/rendering
    pub video: sdl2::VideoSubsystem,
    // pub window: sdl2::video::Window,
    pub window: glium_sdl2::SDL2Facade,
    // gui
    pub imgui: imgui::ImGui,
    pub imgui_renderer: imgui_glium_renderer::Renderer,
    // audio
    pub audio: sdl2::AudioSubsystem,
    pub audio_spec: sdl2::audio::AudioSpecDesired,
    // event handlers
    pub event_pump: RefCell<sdl2::EventPump>,
    // pub last_event: RefCell<Option<sdl2::event::Event>>,
    // controllers
    pub controller: sdl2::GameControllerSubsystem,
    pub controllers: RefCell<[Option<sdl2::controller::GameController>; MAX_CTL]>,
    pub controller_count: u32,
    // testing
}

impl Sdl {
    pub fn new(width: u32, height: u32) -> Result<Sdl, Error> {
        // -- load SDL2 contexts
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video.window("Novluno", width, height)
            .position_centered()
            // .resizable()
            .opengl()
            .build_glium()?;
        let controller = context.game_controller()?;
        let controllers = RefCell::new([None, None, None, None]);
        let event_pump = RefCell::new(context.event_pump()?);
        let audio = context.audio()?;
        let audio_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(2),
            samples: Some(4),
        };

        // -- setup OpenGL
        // gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

        // -- setup ImGui
        let mut imgui = imgui::ImGui::init();
        let imgui_renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &window).unwrap();

        // -- Create SDL state object
        let sdl = Sdl {
            context,
            video,
            window,
            event_pump,
            audio,
            imgui,
            imgui_renderer,
            audio_spec,
            controller,
            controllers,
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
        &mut self,
        game: &mut Game
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
                    Event::MouseMotion { .. } => (),
                    Event::ControllerDeviceAdded { which: index, .. } => {
                        println!("{:?}: {:?}", new_event, index);
                        self.add_game_controller(index).unwrap();
                    }
                    Event::ControllerDeviceRemoved { which: index, .. } => {
                        println!("{:?}: {:?}", new_event, index);
                        self.remove_game_controller(index).unwrap();
                    }
                    Event::JoyDeviceAdded { .. } => (),
                    _ => {
                        println!("{:?}", new_event);
                    }
                }
            }
            last_event = Some(new_event);
        } // end while new SDL event

        // update imgui vars
        // -- mouse
        let sdl_mouse = sdl2::mouse::MouseState::new(&event_pump);
        let mouse_x: f32 = sdl_mouse.x() as f32;
        let mouse_y: f32 = sdl_mouse.y() as f32;
        self.imgui.set_mouse_pos(mouse_x, mouse_y);
        self.imgui.set_mouse_down(&[
            sdl_mouse.left(),
            sdl_mouse.middle(),
            sdl_mouse.right(),
            sdl_mouse.x1(),
            sdl_mouse.x2()]);
    }

    pub fn render(&mut self, game: &Game, dt: f32) {
        // start frame
        let mut target = self.window.draw();

        // draw background color
        target.clear_color(0.4, 0.7, 1.0, 1.0);

        // drawing a triangle
        {
            let vertex_buffer = {
                #[derive(Copy, Clone)]
                struct Vertex {
                    pos: [f32; 2],
                    color: [f32; 3],
                }
                implement_vertex!(Vertex, pos, color);
                glium::VertexBuffer::new(&self.window, &[
                    Vertex { pos: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                    Vertex { pos: [0.0, 0.5], color: [0.0, 0.0, 1.0] },
                    Vertex { pos: [0.5, -0.5], color: [1.0, 1.0, 0.0] },
                ]).unwrap()
            };

            let index_buffer = glium::IndexBuffer::new(
                &self.window,
                PrimitiveType::TrianglesList,
                &[0u16, 1, 2]
            ).unwrap();

            let program = program!(&self.window,
                140 => {
                    vertex: "
                        #version 140
                        uniform mat4 matrix;
                        in vec2 pos;
                        in vec3 color;
                        out vec4 vColor;
                        void main() {
                            gl_position = vec4(pos, 0.0, 1.0) * matrix;
                            vColor = color;
                        }
                    ",
                    fragment: "
                        #version 140
                        in vec3 vColor;
                        out vec4 f_color;
                        void main() {
                            f_color = vec4(vColor, 1.0);
                        }
                    "
                },

                110 => {
                    vertex: "
                        #version 110
                        uniform mat4 matrix;
                        attribute vec2 pos;
                        attribute vec3 color;
                        varying vec3 vColor;
                        void main() {
                            gl_Position = vec4(pos, 0.0, 1.0) * matrix;
                            vColor = color;
                        }
                    ",

                    fragment: "
                        #version 110
                        varying vec3 vColor;
                        void main() {
                            gl_FragColor = vec4(vColor, 1.0);
                        }
                    ",
                },

                100 => {
                    vertex: "
                        #version 100
                        uniform lowp mat4 matrix;
                        attribute lowp vec2 pos;
                        attribute lowp vec3 color;
                        varying lowp vec3 vColor;
                        void main() {
                            gl_Position = vec4(pos, 0.0, 1.0) * matrix;
                            vColor = color;
                        }
                    ",

                    fragment: "
                        #version 100
                        varying lowp vec3 vColor;
                        void main() {
                            gl_FragColor = vec4(vColor, 1.0);
                        }
                    ",
                },
            ).unwrap();

            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ]
            };

            target.draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default()
            ).unwrap();
        }

        // draw gui
        let dim = target.get_dimensions();
        let ui = self.imgui.frame(dim, dim, dt);
        gui::show_gui_test(&ui);
        self.imgui_renderer.render(&mut target, ui).unwrap();

        // finish frame
        target.finish().unwrap();
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
