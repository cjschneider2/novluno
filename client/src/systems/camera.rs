use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::asset_loader::map_assset_loader::MapAsset;
use crate::constants::{TILE_H, TILE_W};
use crate::state::State;
use crate::systems::player::Player;

#[derive(Component)]
pub struct MapCamera;

#[derive(Default, Resource)]
pub struct MapBounds {
    pub width_tiles:  i32,
    pub height_tiles: i32,
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MapCamera));
}

pub fn update_map_bounds(
    state: Res<State>,
    maps:  Res<Assets<MapAsset>>,
    mut bounds: ResMut<MapBounds>,
) {
    if let Some((_, h)) = state.data.maps.iter().next() {
        if let Some(map) = maps.get(h.id()) {
            bounds.width_tiles  = map.0.size_x() as i32;
            bounds.height_tiles = map.0.size_y() as i32;
        }
    }
}

pub fn camera_follow(
    player_q:  Query<&Transform, (With<Player>, Without<MapCamera>)>,
    mut cam_q: Query<&mut Transform, With<MapCamera>>,
    window_q:  Query<&Window, With<PrimaryWindow>>,
    bounds:    Res<MapBounds>,
) {
    let Ok(player_tf) = player_q.single() else { return };
    let Ok(mut cam_tf) = cam_q.single_mut() else { return };
    let Ok(window) = window_q.single() else { return };

    let half_w = window.width()  / 2.0;
    let half_h = window.height() / 2.0;

    let px = player_tf.translation.x;
    let py = player_tf.translation.y;

    let target_x = if bounds.width_tiles == 0 {
        px
    } else {
        let map_w = bounds.width_tiles as f32 * TILE_W;
        let x_min = TILE_W + half_w;
        let x_max = map_w - TILE_W - half_w;
        if x_min > x_max {
            map_w / 2.0
        } else {
            px.clamp(x_min, x_max)
        }
    };

    let target_y = if bounds.height_tiles == 0 {
        py
    } else {
        let map_h = bounds.height_tiles as f32 * TILE_H;
        let y_min = -map_h + TILE_H + half_h;
        let y_max = -TILE_H - half_h;
        if y_min > y_max {
            -(map_h / 2.0)
        } else {
            py.clamp(y_min, y_max)
        }
    };

    cam_tf.translation.x = target_x.round();
    cam_tf.translation.y = target_y.round();
}
