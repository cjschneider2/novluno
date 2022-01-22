mod scene;
mod character;

use std::path::{ PathBuf };

use core_compat::entity::sprite_type::SpriteType;
use core_compat::entity::rmd_type::RmdType;

use crate::sdl::Sdl;

use crate::resource_manager::map_manager::MapManager;
use crate::resource_manager::data_manager::DataManager;
use crate::resource_manager::sprite_manager::SpriteManager;
use crate::resource_manager::list_manager::ListManager;
use crate::resource_manager::list_manager::ListType;

use crate::error::Error;

use self::character::Player;

// public interface

pub mod input;

pub struct State {
    pub player: character::Player,
    pub map: usize,
    pub map_off: (i32, i32),
}

pub struct Game {
    // game and input state
    pub window: (i32, i32),
    pub state: State,
    pub input: input::Input,
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

        path_data.push("data/DATAs");
        path_map.push("data/DATAs/Map");
        path_sprite.push("data/RLEs");

        let map_manager = MapManager::new(&path_map);
        let data_manager = DataManager::new(&path_data);
        let sprite_manager = SpriteManager::new(&path_sprite);
        let list_manager = ListManager::new(&path_sprite).unwrap();

        Game {
            // window state
            window: (800, 600),

            // game and input state
            state: State {
                player: Player::new(),
                map: 0,
                map_off: (-24, -48),
            },
            input: input::Input::new(),

            // data managers
            map_manager,
            data_manager,
            sprite_manager,
            list_manager,
        }
    }

    pub fn update(&mut self) {
        if self.input.keyboard.action_up.pressed {
            // self.state.player_y += 1;
            self.state.map_off.1 += 100;
        }
        if self.input.keyboard.action_down.pressed {
            // self.state.player_y -= 1;
            self.state.map_off.1 -= 100;
        }
        if self.input.keyboard.action_right.pressed {
            // self.state.player_x -= 1;
            self.state.map_off.0 -= 100;
        }
        if self.input.keyboard.action_left.pressed {
            // self.state.player_x += 1;
            self.state.map_off.0 += 100;
        }
        if self.input.keyboard.move_down.pressed {
            if self.state.map > 0 {
                self.state.map -= 1;
                self.state.map_off = (-24, -48);
            }
        }
        if self.input.keyboard.move_up.pressed {
            self.state.map += 1;
            self.state.map_off = (-24, -48);
        }
        if let Some(coords) = self.input.should_resize {
            self.window = coords;
        }

        // player movements ( with keyboard )
        if self.input.keyboard.player_up.pressed {
            // self.state.player_y += 1;
            self.state.player.position.1 -= 5;
        }
        if self.input.keyboard.player_down.pressed {
            // self.state.player_y -= 1;
            self.state.player.position.1 += 5;
        }
        if self.input.keyboard.player_right.pressed {
            // self.state.player_x -= 1;
            self.state.player.position.0 -= 5;
        }
        if self.input.keyboard.player_left.pressed {
            // self.state.player_x += 1;
            self.state.player.position.0 += 5;
        }
    }

    pub fn load_chr(&mut self, _chr_number: usize, _sdl: &mut Sdl) -> Result<(), Error> {

        Ok(())
    }

    pub fn load_map(&mut self, map_number: usize, sdl: &mut Sdl) -> Result<(), Error> {
        // load the map data
        self.map_manager.load_map(map_number)?;
        let map = self.map_manager.get_map(map_number)?;
        // debug
        let mut tile_x = 0;
        let mut _tile_y = 0;
        let tile_stride = map.size_x();
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
                if let Ok(rmd) = self.data_manager.get_data(RmdType::Object,file) {
                    match rmd.get_entry(index) {
                        Some(entry) => {
                            // -- load images
                            for img in entry.images() {
                                // debug
                                // if (img.render_z != 2) && (img.render_z != 0) {
                                //     println!("({:3}, {:3})   dest_x: {:3}, dest_y: {:3}, z: {}",
                                //              tile_x, tile_y, img.dest_x, img.dest_y, img.render_z);
                                // }

                                for id in img.image_id.iter() {
                                    let idx = *id as usize;
                                    let item = obj_list.get_item(idx).unwrap();
                                    let _sprite =
                                        self.sprite_manager
                                            .get_sprite_entry(&item.entry,
                                                              SpriteType::Object,
                                                              sdl)?;
                                }
                            }
                        },
                        None => {
                            println!("failed to get rmd entry for map object");
                            println!("file:  {}", file);
                            println!("index: {}", index);
                        }
                    }

                    // -- load animations
                    for _ani in rmd.animations.iter() {
                        // todo
                    }
                } else {
                    continue;
                }
            }
            // -- map tile sprites
            let tle_entry = map_tile.tle_rmd_entry;
            if tle_entry.file() != 0 {
                let file = tle_entry.file() as usize;
                let index = tle_entry.index() as usize;
                if let Ok(rmd) = self.data_manager.get_data(RmdType::Tile, file) {
                    match rmd.get_entry(index) {
                        Some(entry) => {
                            for img in entry.images() {
                                for id in img.image_id.iter() {
                                    let item = tle_list.get_item(*id as usize).unwrap();
                                    let _sprite = self.sprite_manager.get_sprite_entry(&item.entry, SpriteType::Tile, sdl)?;
                                }
                            }
                        },
                        None => {
                            println!("failed to get rmd entry for map tile");
                            println!("file:  {}", file);
                            println!("index: {}", index);
                        }
                    }
                } else {
                    continue;
                }
            }

            // debugging
            {
                tile_x += 1;
                if tile_x >= tile_stride {
                    tile_x = 0;
                    _tile_y += 1;
                }
                // println!("map_tile.collision: 0x{:2x}", map_tile.collision);
            }
        }

        println!("loaded map: {}", map_number);
        println!("X: {}", map.size_x());
        println!("Y: {}", map.size_y());

        Ok(())
    }

    pub fn get_mut_keyboard(&mut self) -> &mut input::Controller {
        &mut self.input.keyboard
    }
}
