
pub mod input;

pub struct State {
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

    pub fn update(&mut self) {
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
    }

    pub fn get_mut_keyboard(&mut self) -> &mut input::Controller {
        &mut self.input.keyboard
    }
}
