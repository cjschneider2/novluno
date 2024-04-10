use bevy::prelude::*;
use core_compat::entity::map::Map;

use crate::constants::{COLLISION_INDICATOR_SIZE, TILE_H, TILE_W};

#[derive(Resource, Default)]
pub struct CollisionGrid {
    pub width:  u32,
    pub height: u32,
    /// Flat array of blocked flags, indexed by `(y * width + x) * 2 + tile_pos`.
    pub blocked: Vec<bool>,
}

impl CollisionGrid {
    pub fn build(map: &Map) -> Self {
        let w = map.size_x() as usize;
        let h = map.size_y() as usize;
        let mut blocked = vec![false; w * h * 2];

        for y in 0..h {
            for x in 0..w {
                let tile = &map.tiles[y * w + x];

                // First row and warp == 248 are always fully blocked (matches JS).
                let collision = if y == 0 || tile.warp == 248 {
                    0u32
                } else {
                    tile.collision % 128
                };

                let base = (y * w + x) * 2;
                match collision {
                    0  => { blocked[base] = true; blocked[base + 1] = true; }
                    24 => { blocked[base + 1] = true; }   // lower-right sub-tile blocked
                    96 => { blocked[base] = true; }        // upper-left sub-tile blocked
                    _  => {}
                }
            }
        }

        CollisionGrid { width: map.size_x(), height: map.size_y(), blocked }
    }

    pub fn is_blocked(&self, x: i32, y: i32, tile_pos: u8) -> bool {
        if self.blocked.is_empty() { return false; }
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return true;
        }
        let base = (y as usize * self.width as usize + x as usize) * 2;
        self.blocked[base + tile_pos as usize]
    }
}

#[derive(Resource, Default)]
pub struct CollisionOverlaySettings {
    pub show: bool,
}

/// Draws colored gizmo rects over every blocked sub-tile.
///
/// tilePos=0 (upper-left) → red,  tilePos=1 (lower-right) → orange.
/// Only blocked sub-tiles are drawn; walkable tiles produce no gizmos.
pub fn draw_collision_overlay(
    settings: Res<CollisionOverlaySettings>,
    grid: Res<CollisionGrid>,
    mut gizmos: Gizmos,
) {
    if !settings.show || grid.blocked.is_empty() { return; }

    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
            let b0 = grid.is_blocked(x, y, 0);
            let b1 = grid.is_blocked(x, y, 1);
            if !b0 && !b1 { continue; }

            let base_x = x as f32 * TILE_W;
            let base_y = -(y as f32 * TILE_H);

            if b0 {
                // upper-left sub-tile center
                let center = Vec2::new(base_x + 18.0, base_y - 12.0);
                gizmos.rect_2d(
                    Isometry2d::from_translation(center),
                    COLLISION_INDICATOR_SIZE,
                    Color::srgba(1.0, 0.2, 0.2, 0.7),
                );
            }
            if b1 {
                // lower-right sub-tile center
                let center = Vec2::new(base_x + 42.0, base_y - 24.0);
                gizmos.rect_2d(
                    Isometry2d::from_translation(center),
                    COLLISION_INDICATOR_SIZE,
                    Color::srgba(1.0, 0.5, 0.1, 0.7),
                );
            }
        }
    }
}
