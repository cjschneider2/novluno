pub mod camera;
pub mod collision;
pub mod debug_marker;
pub mod debug_overlay;
pub mod fonts;
pub mod map_spawn;
pub mod player;

use bevy::math::{Rect, Vec2};

/// Clips a sprite's source rect against its atlas bounds after adjusting for the stored
/// sprite offset. Returns `(clipped_rect, x_diff, y_diff)` where `x/y_diff` are the
/// positive pixel corrections to add to the destination position, or `None` when the
/// clipped rect has zero or negative area.
pub fn clip_sprite_src(
    source_x1: i32, source_y1: i32,
    source_x2: i32, source_y2: i32,
    x_off: i32, y_off: i32,
    width: i32, height: i32,
) -> Option<(Rect, i32, i32)> {
    let x1 = source_x1 - x_off;
    let y1 = source_y1 - y_off;
    let x2 = source_x2 - x_off;
    let y2 = source_y2 - y_off;
    let x_diff = (-x1).max(0);
    let y_diff = (-y1).max(0);
    let src = Rect {
        min: Vec2::new(x1.max(0) as f32, y1.max(0) as f32),
        max: Vec2::new(x2.clamp(0, width) as f32, y2.clamp(0, height) as f32),
    };
    if src.max.x > src.min.x && src.max.y > src.min.y {
        Some((src, x_diff, y_diff))
    } else {
        None
    }
}
