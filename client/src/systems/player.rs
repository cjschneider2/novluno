use std::collections::HashSet;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::asset_loader::list_asset_loader::ListAsset;
use crate::asset_loader::rle_asset_loader::{RleAsset, SpriteKind};
use crate::asset_loader::rmd_asset_loader::RmdAsset;
use crate::constants::{MOVE_COOLDOWN, PLAYER_ANIM_FPS, SHADOW_ALPHA, TILE_H, TILE_W};
use crate::systems::clip_sprite_src;
use crate::resources::sprite_cache::SpriteCache;
use crate::state::State;
use crate::systems::collision::CollisionGrid;

// ── Enums ─────────────────────────────────────────────────────────────────────

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    DownLeft  = 1,
    Down      = 2,
    DownRight = 3,
    Left      = 4,
    #[default]
    Right     = 6,
    UpLeft    = 7,
    Up        = 8,
    UpRight   = 9,
}

impl Direction {
    pub fn as_usize(self) -> usize { self as usize }

    pub fn name(self) -> &'static str {
        match self {
            Self::DownLeft  => "DownLeft",
            Self::Down      => "Down",
            Self::DownRight => "DownRight",
            Self::Left      => "Left",
            Self::Right     => "Right",
            Self::UpLeft    => "UpLeft",
            Self::Up        => "Up",
            Self::UpRight   => "UpRight",
        }
    }
}

/// Animation base index values 
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharAction {
    Walking = 0,
    Run     = 10,
    #[default]
    Idle    = 60,
}

impl CharAction {
    pub fn anim_base(self) -> usize { self as usize }

    pub fn name(self) -> &'static str {
        match self {
            Self::Walking => "Walking",
            Self::Run     => "Run",
            Self::Idle    => "Idle",
        }
    }
}

// ── Components / Resources ────────────────────────────────────────────────────

#[derive(Component)]
pub struct Player {
    /// Character type index (0 = Philar, 1 = Azlar, … 9 = Aurello).
    pub char_enum: u32,
    /// Tile-grid position.
    pub tile_x: i32,
    pub tile_y: i32,
    /// Sub-tile slot (0 = upper-left offset, 1 = lower-right offset).
    pub tile_pos: u8,
    pub direction: Direction,
    pub action: CharAction,
    /// Current frame index within the animation sequence.
    pub frame_idx: usize,
    /// Per-frame animation advance timer.
    pub timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            char_enum: 0,
            tile_x: 5,
            tile_y: 5,
            tile_pos: 0,
            direction: Direction::default(),
            action: CharAction::default(),
            frame_idx: 0,
            timer: Timer::new(Duration::from_secs_f32(1.0 / PLAYER_ANIM_FPS), TimerMode::Repeating),
        }
    }
}

/// Marker for spawned character sub-sprite entities (children of the Player entity).
#[derive(Component)]
pub struct CharSprite {
    /// Semantic layer from `rmd.layer` (RmdImage::empty_2):
    /// 0=normal/body, 1=skill FX, 2=shadow, 3=clothes, 4=hair, 5/6=weapon FX
    pub layer: i32,
    /// Bitfield from `rmd.drawtype` (RmdImage::draw_type):
    /// 16=skills, 32=full body, 64=shadow-blend, 128=weapons, 512=hair-tint, 1024=shirt-tint
    pub draw_type: i32,
    /// Fine-grained z-sort order within a layer (RmdImage::render_z).
    pub render_z: i32,
}

#[derive(Default, PartialEq)]
enum CharPhase {
    #[default]
    Init,
    WaitingForSprites,
    Done,
}

#[derive(Default, Resource)]
pub struct CharSpawnState {
    phase: CharPhase,
    /// Which char_enum these loaded assets belong to.
    loaded_char: u32,
    rle_handles: Vec<Handle<RleAsset>>,
    /// True when frame_idx or direction changed and CharSprites need re-spawning.
    pub dirty: bool,
}

impl CharSpawnState {
    /// Full reset — forces asset reload for the current char_enum.
    pub fn force_reload(&mut self) {
        self.phase = CharPhase::default();
        self.rle_handles.clear();
        self.dirty = false;
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum CharDebugColorBy {
    #[default]
    Layer,     // semantic layer enum (0=body, 2=shadow, 3=clothes, 4=hair, …)
    DrawType,  // drawtype bitfield (32=body, 64=shadow-blend, 512=hair-tint, …)
    RenderZ,   // fine z-sort index
}

#[derive(Default, Resource)]
pub struct CharDebug {
    pub show_layers: bool,
    pub color_by: CharDebugColorBy,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Converts tile-grid position + sub-tile slot to Bevy world-space pixels.
pub fn tile_to_world(tile_x: i32, tile_y: i32, tile_pos: u8) -> Vec2 {
    let (ox, oy) = if tile_pos == 1 { (36.0_f32, 18.0_f32) } else { (12.0_f32, 6.0_f32) };
    Vec2::new(
        tile_x as f32 * TILE_W + ox,
        -(tile_y as f32 * TILE_H + oy),
    )
}

/// Returns the Z value for the player entity at the given tile row.
///
/// Matches the object Z formula (`1.0 + tile_y + sub_layer / TILE_H`) so the
/// character sorts correctly in front of/behind objects at the same depth.
pub fn player_z(tile_y: i32) -> f32 {
    1.0 + tile_y as f32 + 0.5   // 0.5 puts the character mid-layer within its tile row
}

fn direction_from_delta(dx: i32, dy: i32) -> Direction {
    match (dx.signum(), dy.signum()) {
        ( 1,  0) => Direction::Right,
        (-1,  0) => Direction::Left,
        ( 0, -1) => Direction::Up,
        ( 0,  1) => Direction::Down,
        ( 1, -1) => Direction::UpRight,
        (-1, -1) => Direction::UpLeft,
        ( 1,  1) => Direction::DownRight,
        _        => Direction::DownLeft,
    }
}

// ── Systems ───────────────────────────────────────────────────────────────────

pub fn spawn_player(mut commands: Commands) {
    let player = Player::default();
    let world = tile_to_world(player.tile_x, player.tile_y, player.tile_pos);
    let z = player_z(player.tile_y);
    commands.spawn((
        Transform::from_xyz(world.x, world.y, z),
        Visibility::Visible,
        player,
    ));
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    state: Res<State>,
    maps: Res<Assets<crate::asset_loader::map_assset_loader::MapAsset>>,
    collision_grid: Res<CollisionGrid>,
    mut move_timer: Local<f32>,
    mut q: Query<(&mut Player, &mut Transform)>,
    mut char_spawn_state: ResMut<CharSpawnState>,
) {
    let Ok((mut player, mut transform)) = q.single_mut() else { return };

    // Determine map bounds for clamping (1-tile margin on each edge).
    let (map_w, map_h) = state
        .data
        .maps
        .values()
        .next()
        .and_then(|h| maps.get(h.id()))
        .map(|m| (m.0.size_x() as i32, m.0.size_y() as i32))
        .unwrap_or((50, 50));
    let min_x = 1;
    let max_x = (map_w - 2).max(1);
    let min_y = 1;
    let max_y = (map_h - 2).max(1);

    *move_timer -= time.delta_secs();

    let mut dx = 0i32;
    let mut dy = 0i32;

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::KeyK) { dy -= 1; }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::KeyJ) { dy += 1; }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::KeyH) { dx -= 1; }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::KeyL) { dx += 1; }

    if dx != 0 || dy != 0 {
        let new_dir = direction_from_delta(dx, dy);
        let dir_changed = new_dir != player.direction;
        if dir_changed {
            player.direction = new_dir;
            char_spawn_state.dirty = true;
        }

        if *move_timer <= 0.0 {
            let new_x = (player.tile_x + dx).clamp(min_x, max_x);
            let new_y = (player.tile_y + dy).clamp(min_y, max_y);

            if new_x != player.tile_x || new_y != player.tile_y {
                if !collision_grid.is_blocked(new_x, new_y, player.tile_pos) {
                    player.tile_x = new_x;
                    player.tile_y = new_y;
                    let world = tile_to_world(player.tile_x, player.tile_y, player.tile_pos);
                    transform.translation.x = world.x;
                    transform.translation.y = world.y;
                    transform.translation.z = player_z(player.tile_y);
                }
                *move_timer = MOVE_COOLDOWN;
            }
        }

        if player.action != CharAction::Walking {
            player.action = CharAction::Walking;
            player.frame_idx = 0;
            char_spawn_state.dirty = true;
        }
    } else if player.action != CharAction::Idle {
        player.action = CharAction::Idle;
        player.frame_idx = 0;
        char_spawn_state.dirty = true;
    }
}

/// Loads character RMD/list assets on demand when `char_enum` changes via the debug panel.
pub fn load_char_assets(
    player_q: Query<&Player>,
    mut state: ResMut<State>,
    asset_server: Res<AssetServer>,
) {
    let Ok(player) = player_q.single() else { return };
    let ce = player.char_enum;
    if !state.data.chr_rmds.contains_key(&ce) {
        let rmd_handle = asset_server.load(format!("data/DATAs/Chr/chr{ce:05}.rmd"));
        state.data.chr_rmds.insert(ce, rmd_handle);
    }
    if !state.data.chr_lists.contains_key(&ce) {
        let list_handle = asset_server.load(format!("data/RLEs/Chr/c{ce:02}.lst"));
        state.data.chr_lists.insert(ce, list_handle);
    }
}

/// Advances the per-frame animation timer and marks `CharSpawnState` dirty when the frame
/// index advances.
pub fn advance_char_anim(
    time: Res<Time>,
    rmd_assets: Res<Assets<RmdAsset>>,
    state: Res<State>,
    mut q: Query<&mut Player>,
    mut char_spawn_state: ResMut<CharSpawnState>,
) {
    let Ok(mut player) = q.single_mut() else { return };

    let Some(rmd_handle) = state.data.chr_rmds.get(&player.char_enum) else { return };
    let Some(rmd) = rmd_assets.get(rmd_handle) else { return };

    player.timer.tick(time.delta());
    if !player.timer.just_finished() { return; }

    let anim_key = player.action.anim_base() + player.direction.as_usize();
    let Some(anim) = rmd.0.animations.get(anim_key) else { return };
    let frame_count = anim.frames().len();
    if frame_count == 0 { return; }

    let new_frame = (player.frame_idx + 1) % frame_count;
    if new_frame != player.frame_idx {
        player.frame_idx = new_frame;
        char_spawn_state.dirty = true;
    }
}

/// Manages loading and spawning of character sub-sprite entities.
///
/// Phases:
///   Init            → collect needed RLE file numbers, load them, → WaitingForSprites
///   WaitingForSprites → once all current-frame sprites are cached, spawn, → Done
///   Done + dirty    → re-spawn for the new frame/direction
///   char_enum changed → force_reload back to Init
pub fn spawn_char_sprites(
    state: Res<State>,
    mut char_spawn_state: ResMut<CharSpawnState>,
    rmd_assets: Res<Assets<RmdAsset>>,
    list_assets: Res<Assets<ListAsset>>,
    sprite_cache: Res<SpriteCache>,
    player_q: Query<(Entity, &Player)>,
    char_sprite_q: Query<Entity, With<CharSprite>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok((player_entity, player)) = player_q.single() else { return };

    // If char_enum changed, force a full reload.
    if char_spawn_state.phase == CharPhase::Done
        && char_spawn_state.loaded_char != player.char_enum
    {
        char_spawn_state.force_reload();
    }

    // ── Phase: Init ───────────────────────────────────────────────────────────
    if char_spawn_state.phase == CharPhase::Init {
        let Some(rmd_handle) = state.data.chr_rmds.get(&player.char_enum) else { return };
        let Some(rmd) = rmd_assets.get(rmd_handle) else { return };
        let Some(chr_list) = state.data.chr_lists.get(&player.char_enum)
            .and_then(|h| list_assets.get(h.id())) else { return };

        // Collect all RLE file numbers needed for every frame of the current animation.
        let anim_key = player.action.anim_base() + player.direction.as_usize();
        let mut rle_file_nums: HashSet<u32> = HashSet::new();

        if let Some(anim) = rmd.0.animations.get(anim_key) {
            for &frame_ptr in anim.frames() {
                let entry_idx = frame_ptr.max(0) as usize;
                if let Some(entry) = rmd.0.get_entry(entry_idx) {
                    for img in entry.images() {
                        for &id in &img.image_id {
                            if let Some(item) = chr_list.get_item(id as usize) {
                                rle_file_nums.insert(item.entry.file());
                            }
                        }
                    }
                }
            }
        }

        let ce = player.char_enum;
        char_spawn_state.rle_handles = rle_file_nums
            .iter()
            .map(|&n| asset_server.load(
                format!("data/RLEs/Chr/C{ce:02}/c{ce:02}{n:05}.rle"),
            ))
            .collect();
        char_spawn_state.loaded_char = ce;
        char_spawn_state.phase = CharPhase::WaitingForSprites;
        char_spawn_state.dirty = false;
        return;
    }

    // ── Phase: WaitingForSprites ──────────────────────────────────────────────
    if char_spawn_state.phase == CharPhase::WaitingForSprites {
        if !current_frame_sprites_ready(player, &state, &rmd_assets, &list_assets, &sprite_cache) {
            return;
        }
        do_spawn(
            &mut commands,
            player_entity,
            player,
            &state,
            &rmd_assets,
            &list_assets,
            &sprite_cache,
            &char_sprite_q,
        );
        char_spawn_state.phase = CharPhase::Done;
        char_spawn_state.dirty = false;
        return;
    }

    // ── Phase: Done + dirty ───────────────────────────────────────────────────
    if char_spawn_state.phase == CharPhase::Done && char_spawn_state.dirty {
        if !current_frame_sprites_ready(player, &state, &rmd_assets, &list_assets, &sprite_cache) {
            // Frame data not loaded yet (different animation) — go back to Init to load it.
            char_spawn_state.force_reload();
            return;
        }
        do_spawn(
            &mut commands,
            player_entity,
            player,
            &state,
            &rmd_assets,
            &list_assets,
            &sprite_cache,
            &char_sprite_q,
        );
        char_spawn_state.dirty = false;
    }
}

/// Returns true when all sprite cache entries for the current animation frame are available.
fn current_frame_sprites_ready(
    player: &Player,
    state: &State,
    rmd_assets: &Assets<RmdAsset>,
    list_assets: &Assets<ListAsset>,
    sprite_cache: &SpriteCache,
) -> bool {
    let Some(rmd_handle) = state.data.chr_rmds.get(&player.char_enum) else { return false };
    let Some(rmd) = rmd_assets.get(rmd_handle) else { return false };
    let Some(chr_list) = state.data.chr_lists.get(&player.char_enum)
        .and_then(|h| list_assets.get(h.id())) else { return false };

    let anim_key = player.action.anim_base() + player.direction.as_usize();
    let Some(anim) = rmd.0.animations.get(anim_key) else { return false };
    let Some(&frame_ptr) = anim.frames().get(player.frame_idx) else { return false };
    let entry_idx = frame_ptr.max(0) as usize;
    let Some(entry) = rmd.0.get_entry(entry_idx) else { return false };

    for img in entry.images() {
        for &id in &img.image_id {
            if let Some(item) = chr_list.get_item(id as usize) {
                if sprite_cache.get(SpriteKind::Character, &item.entry).is_none() {
                    return false;
                }
            }
        }
    }
    true
}

/// Despawns old CharSprite children and spawns new ones for the current animation frame.
fn do_spawn(
    commands: &mut Commands,
    player_entity: Entity,
    player: &Player,
    state: &State,
    rmd_assets: &Assets<RmdAsset>,
    list_assets: &Assets<ListAsset>,
    sprite_cache: &SpriteCache,
    char_sprite_q: &Query<Entity, With<CharSprite>>,
) {
    // Despawn previous CharSprite children.
    for e in char_sprite_q.iter() {
        commands.entity(e).despawn();
    }

    let Some(rmd_handle) = state.data.chr_rmds.get(&player.char_enum) else { return };
    let Some(rmd) = rmd_assets.get(rmd_handle) else { return };
    let Some(chr_list) = state.data.chr_lists.get(&player.char_enum)
        .and_then(|h| list_assets.get(h.id())) else { return };

    let anim_key = player.action.anim_base() + player.direction.as_usize();
    let Some(anim) = rmd.0.animations.get(anim_key) else { return };
    let Some(&frame_ptr) = anim.frames().get(player.frame_idx) else { return };
    let entry_idx = frame_ptr.max(0) as usize;
    let Some(entry) = rmd.0.get_entry(entry_idx) else { return };

    // Collect spawn data for all sub-sprites in this frame.
    struct SubSprite {
        handle: Handle<Image>,
        src: Rect,
        dest_x: f32,
        dest_y: f32,
        layer: i32,
        draw_type: i32,
        render_z: i32,
    }
    let mut sub_sprites: Vec<SubSprite> = Vec::new();

    for img in entry.images() {
        if img.source_x2 <= img.source_x1 || img.source_y2 <= img.source_y1 {
            continue;
        }
        for &id in &img.image_id {
            if let Some(item) = chr_list.get_item(id as usize) {
                if let Some(cached) = sprite_cache.get(SpriteKind::Character, &item.entry) {
                    let Some((src, x_diff, y_diff)) = clip_sprite_src(
                        img.source_x1, img.source_y1,
                        img.source_x2, img.source_y2,
                        cached.x_off, cached.y_off,
                        cached.width, cached.height,
                    ) else { continue };
                    sub_sprites.push(SubSprite {
                        handle: cached.handle.clone(),
                        src,
                        dest_x: img.dest_x as f32 + x_diff as f32,
                        dest_y: img.dest_y as f32 + y_diff as f32,
                        layer: img.empty_2,   // rmd.layer — semantic layer enum
                        draw_type: img.draw_type, // rmd.drawtype — tint/blend bitfield
                        render_z: img.render_z,
                    });
                }
            }
        }
    }

    if sub_sprites.is_empty() {
        return;
    }

    // Spawn all children in one `with_children` call.
    // layer (rmd.layer / empty_2): 0=body, 1=skill, 2=shadow, 3=clothes, 4=hair, 5/6=weapon
    // render_z used as secondary fine sort within the same layer.
    commands.entity(player_entity).with_children(|parent| {
        for ss in sub_sprites {
            let z = match ss.layer {
                2 => -0.01,                                  // shadow — behind body
                0 =>  0.00 + ss.render_z as f32 * 0.001,   // body
                3 =>  0.01 + ss.render_z as f32 * 0.001,   // clothes — over body
                4 =>  0.02 + ss.render_z as f32 * 0.001,   // hair
                1 =>  0.05 + ss.render_z as f32 * 0.001,   // skill FX
                5 =>  0.06 + ss.render_z as f32 * 0.001,   // weapon FX
                6 =>  0.07 + ss.render_z as f32 * 0.001,
                _ =>  0.00 + ss.render_z as f32 * 0.001,
            };
            let alpha = if ss.layer == 2 { SHADOW_ALPHA } else { 1.0 };
            parent.spawn((
                Sprite {
                    image: ss.handle,
                    rect: Some(ss.src),
                    color: Color::srgba(1.0, 1.0, 1.0, alpha),
                    ..default()
                },
                Anchor::TOP_LEFT,
                Transform::from_xyz(ss.dest_x, -ss.dest_y, z),
                CharSprite { layer: ss.layer, draw_type: ss.draw_type, render_z: ss.render_z },
            ));
        }
    });
}

const DEBUG_PALETTE: [[f32; 3]; 8] = [
    [1.0, 0.3, 0.3], // red
    [0.3, 1.0, 0.3], // green
    [0.3, 0.3, 1.0], // blue
    [1.0, 1.0, 0.3], // yellow
    [0.3, 1.0, 1.0], // cyan
    [1.0, 0.3, 1.0], // magenta
    [1.0, 0.6, 0.0], // orange
    [0.5, 0.9, 0.5], // lime
];

/// Maps any i32 value to a distinct palette color via a multiplicative hash.
pub fn debug_color_for_value(v: i32) -> Color {
    let idx = (v as u32).wrapping_mul(2654435761) as usize % DEBUG_PALETTE.len();
    let [r, g, b] = DEBUG_PALETTE[idx];
    Color::srgb(r, g, b)
}

fn layer_semantic_color(layer: i32) -> Color {
    match layer {
        0 => Color::srgb(1.0, 0.3, 0.3),  // red    — body (normal)
        1 => Color::srgb(0.3, 1.0, 0.3),  // green  — skill FX
        2 => Color::srgb(0.3, 0.3, 1.0),  // blue   — shadow
        3 => Color::srgb(1.0, 1.0, 0.3),  // yellow — clothes
        4 => Color::srgb(0.3, 1.0, 1.0),  // cyan   — hair
        5 => Color::srgb(1.0, 0.3, 1.0),  // magenta — weapon FX
        6 => Color::srgb(1.0, 0.6, 0.0),  // orange  — weapon FX 2
        _ => Color::srgb(0.6, 0.6, 0.6),
    }
}

/// Recolors CharSprite entities when `CharDebug::show_layers` is on.
pub fn update_char_layer_debug(
    char_debug: Res<CharDebug>,
    mut sprite_q: Query<(&CharSprite, &mut Sprite)>,
) {
    for (cs, mut sprite) in sprite_q.iter_mut() {
        sprite.color = if char_debug.show_layers {
            match char_debug.color_by {
                CharDebugColorBy::Layer    => layer_semantic_color(cs.layer),
                CharDebugColorBy::DrawType => debug_color_for_value(cs.draw_type),
                CharDebugColorBy::RenderZ  => debug_color_for_value(cs.render_z),
            }
        } else {
            Color::srgba(1.0, 1.0, 1.0, if cs.layer == 2 { SHADOW_ALPHA } else { 1.0 })
        };
    }
}
