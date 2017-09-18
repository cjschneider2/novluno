
pub mod input;

pub struct State {
    // _actual_ game state
    pub player_x: usize,
    pub player_y: usize,
    pub map: usize,
}

pub struct Game {
    pub state: State,
    pub input: input::Input,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: State {
                player_x: 50,
                player_y: 50,
                map: 0,
            },
            input: input::Input::new(),
        }
    }

    pub fn update_and_render(&mut self) {
        // Update
        if self.input.keyboard.action_up.pressed {
            self.state.player_y += 1;
        }
        if self.input.keyboard.action_down.pressed {
            self.state.player_y -= 1;
        }
        if self.input.keyboard.action_right.pressed {
            self.state.player_x -= 1;
        }
        if self.input.keyboard.action_left.pressed {
            self.state.player_x += 1;
        }

        // Render
        /*
        let _ = render::weird_gradient(&mut self.render_buffer,
                                       self.state.x_offset as u32,
                                       self.state.y_offset as u32);
        let _ = render::player(&mut self.render_buffer, 50, 50);
        */
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
