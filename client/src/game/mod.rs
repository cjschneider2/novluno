use std::path::{ Path, PathBuf };
use core_compat::{ MapManager, DataManager, SpriteManager, ListManager };

pub mod input;

pub struct State {
    pub player_x: usize,
    pub player_y: usize,
    pub map: usize,
}

pub struct Game {
    // game and input state
    pub state: State,
    pub input: input::Input,
    // file paths
    pub path_data: PathBuf,
    pub path_map: PathBuf,
    pub path_sprite: PathBuf,
    // data managers
    pub map_manager: MapManager,
    pub data_manager: DataManager,
    pub sprite_manager: SpriteManager,
    pub list_manager: ListManager,
}

impl Game {
    pub fn new() -> Game {

        // Construct paths
        let mut path_data = PathBuf::new();
        let mut path_map = PathBuf::new();
        let mut path_sprite = PathBuf::new();

        path_data.push("DATAs/");
        path_map.push(("data/DATAs/Map/");
        path_sprite.push("data/RLEs/");

        let map_manager = MapManager::new(&path_map);
        let data_manager = DataManager::new(&path_data);
        let sprite_manager = SpriteManager::new(&path_sprite);
        let list_manager = ListManager::new(&path_sprite).unwrap();

        Game {
            // game and input state
            state: State {
                player_x: 50,
                player_y: 50,
                map: 0,
            },
            input: input::Input::new(),

            // file paths
            path_data,
            path_map,
            path_sprite,

            // data managers
            map_manager,
            data_manager,
            sprite_manager,
            list_manager,
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
