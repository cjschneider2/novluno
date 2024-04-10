use bevy::math::Vec2;

// Isometric tile dimensions in world-space pixels.
pub const TILE_W: f32 = 48.0;
pub const TILE_H: f32 = 24.0;

// Shadow layer opacity.
pub const SHADOW_ALPHA: f32 = 0.4;

// Map number range.
pub const MAX_MAP: usize = 234;
pub const STARTING_MAP: usize = 1;
/// Extra tile margin rendered beyond the visible viewport on every side.
pub const RENDER_MARGIN_TILES: i32 = 1;

// Player movement.
pub const MOVE_COOLDOWN: f32 = 0.12;

// Animation frame rates.
pub const TILE_ANIM_FPS: f32 = 30.0;
pub const PLAYER_ANIM_FPS: f32 = 15.0;

// Sprite loading.
pub const SPRITE_LOAD_TIMEOUT_FRAMES: u32 = 300;

// Collision overlay indicator size in pixels.
pub const COLLISION_INDICATOR_SIZE: Vec2 = Vec2::new(12.0, 12.0);
