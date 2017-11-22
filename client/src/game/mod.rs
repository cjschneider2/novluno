use std::path::{ PathBuf };

//use error::Error;

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
    // data managers
    // pub map_manager: MapManager,
    // pub data_manager: DataManager,
    // pub sprite_manager: SpriteManager,
    // pub list_manager: ListManager,
}

impl Game {
    pub fn new() -> Game {

        // Construct paths
        let mut path_data = PathBuf::new();
        let mut path_map = PathBuf::new();
        let mut path_sprite = PathBuf::new();

        path_data.push("data/DATAs");
        path_map.push("data/DATAs/Map");
        path_sprite.push("data/RLEs");

        // let map_manager = MapManager::new(&path_map);
        // let data_manager = DataManager::new(&path_data);
        // let sprite_manager = SpriteManager::new(&path_sprite);
        // let list_manager = ListManager::new(&path_sprite).unwrap();

        Game {
            // game and input state
            state: State {
                player_x: 50,
                player_y: 50,
                map: 0,
            },
            input: input::Input::new(),

            // data managers
            // map_manager,
            // data_manager,
            // sprite_manager,
            // list_manager,
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

    /*
    pub fn load_map(&mut self, map_number: usize) -> Result<(), Error> {
        // load the map data
        self.map_manager.load_map(map_number)?;
        let map = self.map_manager.get_map(map_number)?;
        // load the tile data
        let obj_list = self.list_manager.get_list(ListType::Object).unwrap();
        let tle_list = self.list_manager.get_list(ListType::Tile).unwrap();
        for map_tile in map.tiles().iter() {
            // load references to the data files
            // -- map tile objects
            let obj_entry = map_tile.obj_rmd_entry;
            if obj_entry.file() != 0 {
                // -- load entry data
                let file = obj_entry.file() as usize;
                let index = obj_entry.index() as usize;
                let rmd = self.data_manager.get_data(RmdType::Object,file)?;
                let entry = rmd.get_entry(index).unwrap();
                // -- load images
                for img in entry.images() {
                    for id in img.get_image_id_list().iter() {
                        let item = obj_list.get_item(*id as usize).unwrap();
                        let _sprite = self.sprite_manager.get_sprite(item.entry, SpriteType::Object)?;
                    }
                }
                // -- load animations
                for _ani in rmd.animations() {
                    // todo
                }
            }
            // -- map tile sprites
            let tle_entry = map_tile.tle_rmd_entry;
            if tle_entry.file() != 0 {
                let file = tle_entry.file() as usize;
                let index = tle_entry.index() as usize;
                let rmd = self.data_manager.get_data(RmdType::Tile, file)?;
                let entry = rmd.get_entry(index).unwrap();
                for img in entry.images() {
                    for id in img.get_image_id_list().iter() {
                        let item = tle_list.get_item(*id as usize).unwrap();
                        let _sprite = self.sprite_manager.get_sprite(item.entry, SpriteType::Tile)?;
                    }
                }
            }
        }

        Ok(())
    }
    */

    pub fn get_mut_keyboard(&mut self) -> &mut input::Controller {
        &mut self.input.keyboard
    }
}
