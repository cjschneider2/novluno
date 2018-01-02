
pub mod input;
pub mod buffer;
pub mod render;

use std::rc::Rc;

use core_compat::MapManager;
use core_compat::SpriteManager;
use core_compat::entity::sprite::Sprite;

pub struct State {
    // _actual_ game state
    pub player_x: usize,
    pub player_y: usize,
    pub map: usize,

    // Experiments
    pub tone_hz: usize,
    pub x_offset: usize,
    pub y_offset: usize,
    pub sprite: Option<Rc<Sprite>>,
}

pub struct Game {
    pub sprites: SpriteManager,
    pub maps: MapManager,
    pub state: State,
    pub render_buffer: buffer::Image,
    pub input: input::Input,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        // for the software render buffer
        let bpp = 4; // bytes per pixel
        let screen_buffer_size = (width * height * bpp) as usize;
        let mut memory = Vec::with_capacity(screen_buffer_size);
        unsafe { memory.set_len(screen_buffer_size); }
        // Paths
        let sqlite_path = ::std::path::Path::new("./rm.sqlite");
        let data_path = ::std::path::Path::new("./data/DATAs/Map/");
        // Struct init...
        Game {
            sprites: SpriteManager::new(sqlite_path),
            maps: MapManager::new(data_path),
            state: State {
                // game
                player_x: 50,
                player_y: 50,
                map: 0,

                // experiments
                tone_hz: 440,
                x_offset: 0,
                y_offset: 0,
                sprite: None,
            },
            render_buffer: buffer::Image {
                memory: memory,
                width: width,
                height: height,
                pitch: width * bpp,
                bytes_per_pixel: bpp,
            },
            input: input::Input::new(),
        }
    }

    pub fn resize_buffer(&mut self, width: u32, height: u32) {
        let bpp = self.render_buffer.bytes_per_pixel;
        let size = (width * height * bpp) as usize;
        self.render_buffer.pitch = width * bpp;
        self.render_buffer.width = width;
        self.render_buffer.height = height;
        self.render_buffer.memory.clear();
        self.render_buffer.memory.reserve(size);
        unsafe { self.render_buffer.memory.set_len(size); }
    }

    pub fn update_and_render(&mut self) {
        // Update
        if self.input.keyboard.action_up.pressed {
            self.state.y_offset += 1;
        }
        if self.input.keyboard.action_down.pressed {
            self.state.y_offset -= 1;
        }
        if self.input.keyboard.action_right.pressed {
            self.state.x_offset -= 1;
        }
        if self.input.keyboard.action_left.pressed {
            self.state.x_offset += 1;
        }

        // Render
        /*
        let _ = render::weird_gradient(&mut self.render_buffer,
                                       self.state.x_offset as u32,
                                       self.state.y_offset as u32);
        let _ = render::player(&mut self.render_buffer, 50, 50);
        */

        let sprite = self.state.sprite.clone().unwrap();
        let _ = render::render_sprite(
            &mut self.render_buffer,
            sprite, 100, 100);
    }

    /*
    pub fn output_sound(
        &mut self,
        sound_buffer: &mut buffer::SoundOutput,
        tone_hz: usize)
    {
        let tone_volume = 500.0;
        let wave_period = (sound_buffer.samples_per_second / tone_hz) as f32;
        for sample in sound_buffer.samples.iter_mut() {
            let sine_value = self.state.t_sine.sin();
            let sample_value = (sine_value * tone_volume) as i16;
            *sample = sample_value;
            self.state.t_sine += 2.0 * PI / wave_period;
            if self.state.t_sine > 2.0 * PI {
                self.state.t_sine -= 2.0 * PI;
            }
        }
    }
    */

    pub fn get_mut_keyboard(&mut self) -> &mut input::Controller {
        &mut self.input.keyboard
    }
}
