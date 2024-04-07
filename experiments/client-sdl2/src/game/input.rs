
pub const MAX_CONTROLLERS: usize = 4;
pub const MAX_MOUSE_BUTTONS: usize = 5;

pub struct Input {
    // control flow inputs
    pub should_quit: bool,
    pub should_resize: Option<(i32, i32)>,
    // mouse inputs
    pub mouse_left: ButtonState,
    pub mouse_right: ButtonState,
    pub mouse_middle: ButtonState,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_z: i32,
    // keyboard inputs
    pub keyboard: Controller,
    // controller inputs
    pub controllers: [Controller; MAX_CONTROLLERS],
}

impl Input {
    pub fn new() -> Input {
        Input {
            should_quit: false,
            should_resize: None,
            mouse_left: ButtonState::new(),
            mouse_right: ButtonState::new(),
            mouse_middle: ButtonState::new(),
            mouse_x: 0,
            mouse_y: 0,
            mouse_z: 0,
            keyboard: Controller::new(),
            controllers: [Controller::new(); 4],
        }
    }
}

#[derive(Clone, Copy)]
pub struct ButtonState {
    pub half_transition_count: usize,
    pub pressed: bool,
}

impl ButtonState {
    pub fn new() -> ButtonState {
        ButtonState {
            half_transition_count: 0,
            pressed: false,
        }
    }

    pub fn key_press(&mut self, is_down: bool) {
        assert!(self.pressed != is_down);
        self.pressed = is_down;
        self.half_transition_count += 1;
    }
}

#[derive(Clone, Copy)]
pub struct Controller {
    pub is_connected: bool,
    pub is_analog: bool,
    pub stick_avg_x: f32,
    pub stick_avg_y: f32,
    pub move_up: ButtonState,
    pub move_down: ButtonState,
    pub move_left: ButtonState,
    pub move_right: ButtonState,
    pub action_up: ButtonState,
    pub action_down: ButtonState,
    pub action_left: ButtonState,
    pub action_right: ButtonState,
    pub left_shoulder: ButtonState,
    pub right_shoulder: ButtonState,
    pub btn_back: ButtonState,
    pub btn_start: ButtonState,
    pub player_up: ButtonState,
    pub player_down: ButtonState,
    pub player_left: ButtonState,
    pub player_right: ButtonState,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            is_connected: false,
            is_analog: false,
            stick_avg_x: 0.0,
            stick_avg_y: 0.0,
            move_up: ButtonState::new(),
            move_down: ButtonState::new(),
            move_left: ButtonState::new(),
            move_right: ButtonState::new(),
            action_up: ButtonState::new(),
            action_down: ButtonState::new(),
            action_left: ButtonState::new(),
            action_right: ButtonState::new(),
            left_shoulder: ButtonState::new(),
            right_shoulder: ButtonState::new(),
            btn_back: ButtonState::new(),
            btn_start: ButtonState::new(),
            player_up: ButtonState::new(),
            player_down: ButtonState::new(),
            player_left: ButtonState::new(),
            player_right: ButtonState::new(),
        }
    }
}


