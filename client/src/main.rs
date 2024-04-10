mod config;
mod asset_loader;

use std::path::PathBuf;
use bevy::prelude::*;
use core_compat::entity::list::List;
use core_compat::entity::list_item::ListItem;
use crate::asset_loader::list_asset_loader::ListAssetLoader;
use crate::config::Config;

pub struct CustomDefaultPlugin;
impl Plugin for CustomDefaultPlugin{
    // NOTE: this is needed for working around a bug in wgpu which spams the
    //       logs with errors due to a bug related to vulkan and amd drivers:
    //  SEE: https://github.com/gfx-rs/wgpu/issues/4247
    #[cfg(target_os = "windows")]
    fn build(&self, app: &mut App) {
        use bevy::render::RenderPlugin;
        use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
        app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
            ..default()
        }));
    }
    #[cfg(not(target_os = "windows"))]
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}

#[derive(Default, TypePath, Asset, Debug)]
pub struct ListAsset {
    pub items: Vec<ListItem>
}

#[derive(Resource, Default)]
struct State {
    pub tile_list: Handle<ListAsset>,
    pub printed_dbg: bool,
}

fn load_assets(mut state: ResMut<State>, asset_server: Res<AssetServer>, config: Res<Config>) {
    // TODO: move into assets?
    state.tile_list = asset_server.load("data/RLEs/tle.lst");
    state.printed_dbg = false;
}

fn debug_stuff(mut state: ResMut<State>, list_asset: Res<Assets<ListAsset>>) {
    if !state.printed_dbg {

        if let Some(list) = list_asset.get(&state.tile_list) {
            // dbg!(list);
        }

        state.printed_dbg = true;
    }
}

fn main() {
    App::new()
        .add_plugins(CustomDefaultPlugin)
        .init_asset::<ListAsset>()
        .init_resource::<Config>()
        .init_resource::<State>()
        .init_asset_loader::<ListAssetLoader>()
        .add_systems(PreStartup, config::load_system_config)
        .add_systems(Startup, load_assets)
        .add_systems(Update, debug_stuff)
        .run();
}
