mod asset_loader;
mod config;
mod constants;
mod resources;
mod state;
mod systems;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::asset_loader::list_asset_loader::{ListAsset, ListAssetLoader};
use crate::asset_loader::map_assset_loader::{MapAsset, MapAssetLoader};
use crate::asset_loader::rle_asset_loader::{RleAsset, RleAssetLoader};
use crate::asset_loader::rmd_asset_loader::{RmdAsset, RmdAssetLoader};
use crate::config::Config;
use crate::resources::data_cache::DataCache;
use crate::resources::map_index::MapIndex;
use crate::resources::sprite_cache::{populate_sprite_cache, SpriteCache};
use crate::systems::camera::{camera_follow, spawn_camera, update_map_bounds, MapBounds};
use crate::systems::fonts::setup_fonts;
use crate::systems::debug_marker::spawn_debug_marker;
use crate::systems::debug_overlay::{debug_overlay, draw_debug_gizmos};
use crate::state::sync_frame_limit;
use crate::systems::collision::{draw_collision_overlay, CollisionGrid, CollisionOverlaySettings};
use crate::systems::map_spawn::{animate_sprites, change_map, map_cycle_input, spawn_tiles, MapChangeEvent, SpawnState};
use crate::systems::player::{
    advance_char_anim, load_char_assets, player_movement, spawn_char_sprites, spawn_player,
    update_char_layer_debug, CharDebug, CharSpawnState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0, 0.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(EguiPlugin::default())
        .init_asset::<ListAsset>()
        .init_asset::<MapAsset>()
        .init_asset::<RleAsset>()
        .init_asset::<RmdAsset>()
        .init_resource::<Config>()
        .init_resource::<state::State>()
        .init_resource::<MapIndex>()
        .init_resource::<SpriteCache>()
        .init_resource::<DataCache>()
        .init_resource::<CollisionGrid>()
        .init_resource::<CollisionOverlaySettings>()
        .init_resource::<SpawnState>()
        .init_resource::<CharSpawnState>()
        .init_resource::<CharDebug>()
        .init_resource::<MapBounds>()
        .add_message::<MapChangeEvent>()
        .init_asset_loader::<ListAssetLoader>()
        .init_asset_loader::<MapAssetLoader>()
        .init_asset_loader::<RleAssetLoader>()
        .init_asset_loader::<RmdAssetLoader>()
        .add_systems(PreStartup, config::load_system_config)
        .add_systems(Startup, (state::load_assets, spawn_camera, spawn_player, spawn_debug_marker))
        .add_systems(Update, (
            sync_frame_limit,
            load_char_assets,
            map_cycle_input,
            change_map.after(map_cycle_input),
            populate_sprite_cache,
            spawn_tiles.after(populate_sprite_cache).after(change_map),
            update_map_bounds.after(spawn_tiles),
            animate_sprites,
            player_movement,
            advance_char_anim.after(player_movement),
            spawn_char_sprites.after(advance_char_anim).after(populate_sprite_cache),
            update_char_layer_debug.after(spawn_char_sprites),
            camera_follow.after(player_movement).after(update_map_bounds),
            draw_debug_gizmos,
            draw_collision_overlay,
        ))
        .add_systems(EguiPrimaryContextPass, (setup_fonts, debug_overlay).chain())
        .run();
}
