mod list_files;

use crate::config::list_files::ListType;
use bevy::log::info;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Config {
    pub data_dir: String,
    pub rle_dir: String,
    pub list_paths: Vec<(String, ListType)>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            data_dir: "../Data/DATAs".into(),
            rle_dir: "../Data/RELs".into(),
            list_paths: vec![
                ("bul.lst".into(), ListType::Bullet),
                ("ico.lst".into(), ListType::Icon),
                ("int.lst".into(), ListType::Interface),
                ("tle.lst".into(), ListType::Tile),
                ("obj.lst".into(), ListType::Object),
            ],
        }
    }
}

pub fn load_system_config(mut config: ResMut<Config>) {
    info!("loading configuration");
    /*
    NOTE:
        I'm planning on dynamically loading the config at runtime to make it possible to change the
        config without recompiling but for now it's just a stub...
     */

    info!("using the following paths:");
    info!("Rle Path : ")
}
