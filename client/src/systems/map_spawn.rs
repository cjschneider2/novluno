use std::collections::HashSet;
use std::time::Duration;

use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::asset_loader::list_asset_loader::ListAsset;
use crate::asset_loader::map_assset_loader::MapAsset;
use crate::asset_loader::rle_asset_loader::{RleAsset, SpriteKind};
use crate::asset_loader::rmd_asset_loader::RmdAsset;
use crate::constants::{
    MAX_MAP, SHADOW_ALPHA, SPRITE_LOAD_TIMEOUT_FRAMES, TILE_ANIM_FPS, TILE_H, TILE_W,
};
use crate::systems::clip_sprite_src;
use crate::resources::data_cache::DataCache;
use crate::resources::sprite_cache::SpriteCache;
use crate::state::State;
use crate::systems::collision::CollisionGrid;

#[derive(Component)]
pub struct TileSprite;

#[derive(Component)]
pub struct ObjectSprite;

/// Looping animation frames precomputed at spawn time.
/// Each frame stores the image handle, source rect, and world-space XY position.
/// XY is constant for tiles (no offset), and per-frame for objects where dest_x/dest_y
/// or the sprite's x_off/y_off may differ across animation entries.
#[derive(Component)]
pub struct AnimFrames {
    pub frames: Vec<(Handle<Image>, Rect, Vec2)>,
    pub frame_idx: usize,
    pub timer: Timer,
}

impl AnimFrames {
    fn new(frames: Vec<(Handle<Image>, Rect, Vec2)>) -> Self {
        Self {
            frames,
            frame_idx: 0,
            timer: Timer::new(Duration::from_secs_f32(1.0 / TILE_ANIM_FPS), TimerMode::Repeating),
        }
    }
}

/// Debug metadata attached to every spawned tile/object sprite.
#[derive(Component)]
pub struct DebugInfo {
    pub tile_x: i32,
    pub tile_y: i32,
    pub z: f32,
    pub kind: &'static str,
    /// Rendered pixel width (after custom_size or src_rect cropping).
    pub render_w: f32,
    /// Rendered pixel height.
    pub render_h: f32,
}

#[derive(Message, Clone)]
pub struct MapChangeEvent {
    pub map_number: usize,
}

#[derive(Default, PartialEq)]
pub(crate) enum Phase {
    #[default]
    Init,
    WaitingForRmds,
    WaitingForSprites,
    Done,
}

#[derive(Default, Resource)]
pub struct SpawnState {
    pub phase: Phase,
    pub map_id: Option<AssetId<MapAsset>>,
    // tile layer
    pub tle_rmd_file_nums: HashSet<u32>,
    pub tle_rle_handles: Vec<Handle<RleAsset>>,
    // object layer
    pub obj_rmd_file_nums: HashSet<u32>,
    pub obj_rle_handles: Vec<Handle<RleAsset>>,
    // how many frames we have spent waiting for sprites
    pub frames_waiting: u32,
}

impl SpawnState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Returns the RMD entry indices for every animation frame of a sprite.
///
/// For non-animated entries (static sprites or a one-frame loop) this returns
/// `vec![base]`.  For looping animations the formula is:
///   entry = base + animation_entry_count * animations[0].frames[f]
fn anim_entry_indices(rmd: &RmdAsset, base: usize) -> Vec<usize> {
    if rmd.0.animation_count == 0 || rmd.0.animations.is_empty() {
        return vec![base];
    }
    let stride = rmd.0.animation_entry_count as usize;
    let frames = rmd.0.animations[0].frames();
    if frames.len() <= 1 {
        return vec![base];
    }
    frames
        .iter()
        .map(|&f| base + stride * f.max(0) as usize)
        .collect()
}

pub fn spawn_tiles(
    maps: Res<Assets<MapAsset>>,
    rmd_assets: Res<Assets<RmdAsset>>,
    list_assets: Res<Assets<ListAsset>>,
    sprite_cache: Res<SpriteCache>,
    mut data_cache: ResMut<DataCache>,
    state: Res<State>,
    mut spawn_state: ResMut<SpawnState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_grid: ResMut<CollisionGrid>,
) {
    if spawn_state.phase == Phase::Done {
        return;
    }

    // ── Phase: Init ───────────────────────────────────────────────────────────
    // Wait for the map and both list assets to be ready, then request all RMD
    // files needed by the tile and object layers.
    if spawn_state.phase == Phase::Init {
        let Some((_, map_handle)) = state.data.maps.iter().next() else {
            return;
        };
        let map_id = map_handle.id();
        let Some(map) = maps.get(map_id) else { return };
        let Some(_) = list_assets.get(state.data.tile_list.id()) else { return };
        let Some(_) = list_assets.get(state.data.obj_list.id()) else { return };

        let mut tle_rmd_file_nums = HashSet::new();
        let mut obj_rmd_file_nums = HashSet::new();

        for tile in map.0.tiles() {
            let tf = tile.tle_rmd_entry.file();
            if tf != 0 { tle_rmd_file_nums.insert(tf); }

            let of = tile.obj_rmd_entry.file();
            if of != 0 { obj_rmd_file_nums.insert(of); }
        }

        for &n in &tle_rmd_file_nums {
            let h = asset_server.load(format!("data/DATAs/Tle/tle{n:05}.rmd"));
            data_cache.insert(SpriteKind::Tile, n, h);
        }
        for &n in &obj_rmd_file_nums {
            let h = asset_server.load(format!("data/DATAs/Obj/obj{n:05}.rmd"));
            data_cache.insert(SpriteKind::Object, n, h);
        }

        spawn_state.map_id = Some(map_id);
        spawn_state.tle_rmd_file_nums = tle_rmd_file_nums;
        spawn_state.obj_rmd_file_nums = obj_rmd_file_nums;
        info!(
            "[spawn_tiles] Init → WaitingForRmds: {} tile RMDs, {} obj RMDs requested",
            spawn_state.tle_rmd_file_nums.len(),
            spawn_state.obj_rmd_file_nums.len(),
        );
        spawn_state.phase = Phase::WaitingForRmds;
    }

    // ── Phase: WaitingForRmds ─────────────────────────────────────────────────
    // Once ALL tile and object RMDs are loaded, walk their entries (all animation
    // frames, not just frame 0) to determine which RLE sprite files are needed.
    if spawn_state.phase == Phase::WaitingForRmds {
        let tle_ready = spawn_state.tle_rmd_file_nums.iter().all(|&n| {
            data_cache.get(SpriteKind::Tile, n).and_then(|h| rmd_assets.get(h)).is_some()
        });
        let obj_ready = spawn_state.obj_rmd_file_nums.iter().all(|&n| {
            data_cache.get(SpriteKind::Object, n).and_then(|h| rmd_assets.get(h)).is_some()
        });
        if !tle_ready || !obj_ready { return; }

        let map = maps.get(spawn_state.map_id.unwrap()).unwrap();
        let tile_list = list_assets.get(state.data.tile_list.id()).unwrap();
        let obj_list  = list_assets.get(state.data.obj_list.id()).unwrap();

        let mut tle_rle_file_nums: HashSet<u32> = HashSet::new();
        let mut obj_rle_file_nums: HashSet<u32> = HashSet::new();

        for tile in map.0.tiles() {
            let (tf, ti) = (tile.tle_rmd_entry.file(), tile.tle_rmd_entry.index() as usize);
            tle_rle_file_nums.extend(collect_rle_file_nums(SpriteKind::Tile, tf, ti, &data_cache, &rmd_assets, tile_list));
            let (of, oi) = (tile.obj_rmd_entry.file(), tile.obj_rmd_entry.index() as usize);
            obj_rle_file_nums.extend(collect_rle_file_nums(SpriteKind::Object, of, oi, &data_cache, &rmd_assets, obj_list));
        }

        info!(
            "[spawn_tiles] WaitingForRmds → WaitingForSprites: {} tile RLE files, {} obj RLE files requested",
            tle_rle_file_nums.len(),
            obj_rle_file_nums.len(),
        );
        spawn_state.tle_rle_handles = tle_rle_file_nums.iter()
            .map(|&n| asset_server.load(format!("data/RLEs/Tle/tle{n:05}.rle")))
            .collect();
        spawn_state.obj_rle_handles = obj_rle_file_nums.iter()
            .map(|&n| asset_server.load(format!("data/RLEs/Obj/obj{n:05}.rle")))
            .collect();

        spawn_state.phase = Phase::WaitingForSprites;
    }

    // ── Phase: WaitingForSprites ──────────────────────────────────────────────
    // Each frame, do a readiness check over all tile+object sprites (all animation
    // frames). Once everything is cached, spawn both layers in one shot.
    if spawn_state.phase == Phase::WaitingForSprites {
        let map_id = spawn_state.map_id.unwrap();
        let Some(map) = maps.get(map_id) else { return };
        let Some(tile_list) = list_assets.get(state.data.tile_list.id()) else { return };
        let Some(obj_list)  = list_assets.get(state.data.obj_list.id())  else { return };

        // ── readiness check ───────────────────────────────────────────────────
        let mut all_ready = true;
        'check: for tile in map.0.tiles() {
            let (tf, ti) = (tile.tle_rmd_entry.file(), tile.tle_rmd_entry.index() as usize);
            if !sprites_ready("tile", SpriteKind::Tile, tf, ti, &data_cache, &rmd_assets, tile_list, &sprite_cache) {
                all_ready = false;
                break 'check;
            }
            let (of, oi) = (tile.obj_rmd_entry.file(), tile.obj_rmd_entry.index() as usize);
            if !sprites_ready("obj", SpriteKind::Object, of, oi, &data_cache, &rmd_assets, obj_list, &sprite_cache) {
                all_ready = false;
                break 'check;
            }
        }
        if !all_ready {
            spawn_state.frames_waiting += 1;
            if spawn_state.frames_waiting < SPRITE_LOAD_TIMEOUT_FRAMES {
                return;
            }
            warn!(
                "[spawn_tiles] timed out after {} frames waiting for sprites — spawning with whatever is cached",
                spawn_state.frames_waiting,
            );
        }

        // ── spawn ─────────────────────────────────────────────────────────────
        let stride = map.0.size_x() as i32;
        let mut tile_x = 0i32;
        let mut tile_y = 0i32;

        // Collect objects into a draw list for z-sorting before spawning.
        struct ObjEntry {
            pos: Vec3,
            handle: Handle<Image>,
            src_rect: Rect,
            tile_x: i32,
            tile_y: i32,
            alpha: f32,
            anim_frames: Option<Vec<(Handle<Image>, Rect, Vec2)>>,
        }
        let mut obj_draw: Vec<ObjEntry> = Vec::new();
        let mut tile_sprite_count = 0usize;

        for tile in map.0.tiles() {
            let base_x = tile_x as f32 * TILE_W;
            let base_y = -(tile_y as f32 * TILE_H);

            // ── tile sprites ──────────────────────────────────────────────────
            let (tf, ti) = (tile.tle_rmd_entry.file(), tile.tle_rmd_entry.index() as usize);
            'tile: {
                if tf == 0 { break 'tile; }
                let Some(rmd) = data_cache.get(SpriteKind::Tile, tf).and_then(|h| rmd_assets.get(h)) else { break 'tile };
                let entry_indices = anim_entry_indices(rmd, ti);
                let is_animated = entry_indices.len() > 1;
                let Some(entry) = rmd.0.get_entry(ti) else { break 'tile };

                for (img_idx, img) in entry.images().iter().enumerate() {
                    if img.source_x2 <= img.source_x1 || img.source_y2 <= img.source_y1 {
                        continue;
                    }
                    for (id_idx, &id) in img.image_id.iter().enumerate() {
                        let Some(item) = tile_list.get_item(id as usize) else { continue };
                        let Some(cached) = sprite_cache.get(SpriteKind::Tile, &item.entry) else { continue };
                        let z = 1.0; // (tile_y * stride + tile_x) as f32 * 0.001 + img_idx as f32 * 0.001;
                        let src0 = Rect {
                            min: Vec2::new(img.source_x1 as f32, img.source_y1 as f32),
                            max: Vec2::new(img.source_x2 as f32, img.source_y2 as f32),
                        };

                        let mut ec = commands.spawn((
                            Sprite {
                                image: cached.handle.clone(),
                                rect: Some(src0),
                                custom_size: Some(Vec2::new(TILE_W, TILE_H)),
                                ..default()
                            },
                            Anchor::TOP_LEFT,
                            Transform::from_xyz(base_x, base_y, z),
                            TileSprite,
                            DebugInfo { tile_x, tile_y, z, kind: "tile", render_w: TILE_W, render_h: TILE_H },
                        ));

                        if is_animated {
                            let frames = collect_tile_frames(
                                &entry_indices, img_idx, id_idx,
                                &rmd.0, tile_list, &sprite_cache,
                                base_x, base_y,
                            );
                            if frames.len() > 1 {
                                ec.insert(AnimFrames::new(frames));
                            }
                        }

                        tile_sprite_count += 1;
                    }
                }
            }

            // ── object sprites (collected for z-sort) ─────────────────────────
            let (of, oi) = (tile.obj_rmd_entry.file(), tile.obj_rmd_entry.index() as usize);
            'obj: {
                if of == 0 { break 'obj; }
                let Some(rmd) = data_cache.get(SpriteKind::Object, of).and_then(|h| rmd_assets.get(h)) else { break 'obj };
                let entry_indices = anim_entry_indices(rmd, oi);
                let is_animated = entry_indices.len() > 1;
                let Some(entry) = rmd.0.get_entry(oi) else { break 'obj };

                for (img_idx, img) in entry.images().iter().enumerate() {
                    for (id_idx, &id) in img.image_id.iter().enumerate() {
                        let Some(item) = obj_list.get_item(id as usize) else { continue };
                        let Some(cached) = sprite_cache.get(SpriteKind::Object, &item.entry) else { continue };
                        let Some((src, x_diff, y_diff)) = clip_sprite_src(
                            img.source_x1, img.source_y1,
                            img.source_x2, img.source_y2,
                            cached.x_off, cached.y_off,
                            cached.width, cached.height,
                        ) else { continue };
                        let z = 2.0
                            + tile_y as f32
                            + (img_idx as f32 - img.render_z as f32) / TILE_H;
                        let x = base_x + img.dest_x as f32 + x_diff as f32;
                        let y = base_y - img.dest_y as f32 - y_diff as f32;
                        // JS: layers 1–3 get 40% opacity; 4–6 and 0 get full opacity.
                        let alpha = if (1..=3).contains(&img.empty_2) { SHADOW_ALPHA } else { 1.0 };

                        let anim_frames = if is_animated {
                            let frames = collect_obj_frames(
                                &entry_indices, img_idx, id_idx,
                                &rmd.0, obj_list, &sprite_cache,
                                base_x, base_y,
                            );
                            if frames.len() > 1 { Some(frames) } else { None }
                        } else {
                            None
                        };

                        obj_draw.push(ObjEntry {
                            pos: Vec3::new(x, y, z),
                            handle: cached.handle.clone(),
                            src_rect: src,
                            tile_x,
                            tile_y,
                            alpha,
                            anim_frames,
                        });
                    }
                }
            }

            next_tile(&mut tile_x, &mut tile_y, stride);
        }

        let obj_count = obj_draw.len();
        obj_draw.sort_unstable_by(|a, b| a.pos.z.partial_cmp(&b.pos.z).unwrap());
        for obj in obj_draw {
            let render_w = obj.src_rect.max.x - obj.src_rect.min.x;
            let render_h = obj.src_rect.max.y - obj.src_rect.min.y;
            let z = obj.pos.z;
            let mut ec = commands.spawn((
                Sprite {
                    image: obj.handle,
                    rect: Some(obj.src_rect),
                    color: Color::srgba(1.0, 1.0, 1.0, obj.alpha),
                    ..default()
                },
                Anchor::TOP_LEFT,
                Transform::from_translation(obj.pos),
                ObjectSprite,
                DebugInfo { tile_x: obj.tile_x, tile_y: obj.tile_y, z, kind: "obj", render_w, render_h },
            ));
            if let Some(frames) = obj.anim_frames {
                ec.insert(AnimFrames::new(frames));
            }
        }

        *collision_grid = CollisionGrid::build(&map.0);
        info!(
            "[spawn_tiles] CollisionGrid built: {}×{} ({} blocked sub-tiles)",
            collision_grid.width,
            collision_grid.height,
            collision_grid.blocked.iter().filter(|&&b| b).count(),
        );

        spawn_state.phase = Phase::Done;
        info!(
            "[spawn_tiles] Done: {} tile sprites, {} object sprites",
            tile_sprite_count,
            obj_count,
        );
    }
}

/// Precomputes `(Handle<Image>, Rect, Vec2)` for every animation frame of a tile sprite.
///
/// Tile position never changes between frames (`custom_size` keeps rendered dimensions
/// constant), so every frame carries the same XY (`base_x`, `base_y`).
fn collect_tile_frames(
    entry_indices: &[usize],
    img_idx: usize,
    id_idx: usize,
    rmd: &core_compat::entity::rmd::Rmd,
    tile_list: &ListAsset,
    sprite_cache: &SpriteCache,
    base_x: f32,
    base_y: f32,
) -> Vec<(Handle<Image>, Rect, Vec2)> {
    let xy = Vec2::new(base_x, base_y);
    let mut frames = Vec::with_capacity(entry_indices.len());
    for &eidx in entry_indices {
        if let Some(entry) = rmd.get_entry(eidx) {
            if let Some(img) = entry.images().get(img_idx) {
                if img.source_x2 <= img.source_x1 || img.source_y2 <= img.source_y1 {
                    continue;
                }
                if let Some(&fid) = img.image_id.get(id_idx) {
                    if let Some(item) = tile_list.get_item(fid as usize) {
                        if let Some(cached) = sprite_cache.get(SpriteKind::Tile, &item.entry) {
                            let src = Rect {
                                min: Vec2::new(img.source_x1 as f32, img.source_y1 as f32),
                                max: Vec2::new(img.source_x2 as f32, img.source_y2 as f32),
                            };
                            frames.push((cached.handle.clone(), src, xy));
                        }
                    }
                }
            }
        }
    }
    frames
}

/// Precomputes `(Handle<Image>, Rect, Vec2)` for every animation frame of an object sprite.
///
/// Each frame uses its own sprite's `x_off`/`y_off` (not frame 0's), and recomputes
/// `dest_x`/`dest_y` + `x_diff`/`y_diff` so the stored XY is the correct world position
/// for that frame.  The `animate_sprites` system applies the stored XY to the transform
/// so objects don't jump when `y_diff` changes between frames.
fn collect_obj_frames(
    entry_indices: &[usize],
    img_idx: usize,
    id_idx: usize,
    rmd: &core_compat::entity::rmd::Rmd,
    obj_list: &ListAsset,
    sprite_cache: &SpriteCache,
    base_x: f32,
    base_y: f32,
) -> Vec<(Handle<Image>, Rect, Vec2)> {
    let mut frames = Vec::with_capacity(entry_indices.len());
    for &eidx in entry_indices {
        if let Some(entry) = rmd.get_entry(eidx) {
            if let Some(img) = entry.images().get(img_idx) {
                if let Some(&fid) = img.image_id.get(id_idx) {
                    if let Some(item) = obj_list.get_item(fid as usize) {
                        if let Some(cached) = sprite_cache.get(SpriteKind::Object, &item.entry) {
                            if let Some((src, x_diff, y_diff)) = clip_sprite_src(
                                img.source_x1, img.source_y1,
                                img.source_x2, img.source_y2,
                                cached.x_off, cached.y_off,
                                cached.width, cached.height,
                            ) {
                                let x = base_x + img.dest_x as f32 + x_diff as f32;
                                let y = base_y - img.dest_y as f32 - y_diff as f32;
                                frames.push((cached.handle.clone(), src, Vec2::new(x, y)));
                            }
                        }
                    }
                }
            }
        }
    }
    frames
}

fn collect_rle_file_nums(
    kind: SpriteKind,
    file: u32,
    index: usize,
    data_cache: &DataCache,
    rmd_assets: &Assets<RmdAsset>,
    list: &ListAsset,
) -> Vec<u32> {
    let mut nums = Vec::new();
    if file == 0 { return nums; }
    let Some(rmd) = data_cache.get(kind, file).and_then(|h| rmd_assets.get(h)) else { return nums };
    for entry_idx in anim_entry_indices(rmd, index) {
        if let Some(entry) = rmd.0.get_entry(entry_idx) {
            for img in entry.images() {
                for &id in &img.image_id {
                    if let Some(item) = list.get_item(id as usize) {
                        nums.push(item.entry.file());
                    }
                }
            }
        }
    }
    nums
}

fn sprites_ready(
    label: &str,
    kind: SpriteKind,
    file: u32,
    index: usize,
    data_cache: &DataCache,
    rmd_assets: &Assets<RmdAsset>,
    list: &ListAsset,
    sprite_cache: &SpriteCache,
) -> bool {
    if file == 0 { return true; }
    let Some(rmd) = data_cache.get(kind, file).and_then(|h| rmd_assets.get(h)) else {
        info!("[spawn_tiles] waiting: {label} RMD not ready (file={file})");
        return false;
    };
    for entry_idx in anim_entry_indices(rmd, index) {
        if let Some(entry) = rmd.0.get_entry(entry_idx) {
            for img in entry.images() {
                for &id in &img.image_id {
                    if let Some(item) = list.get_item(id as usize) {
                        if sprite_cache.get(kind, &item.entry).is_none() {
                            info!("[spawn_tiles] waiting: {label} sprite missing (rmd_file={file} rmd_idx={entry_idx} img_id={id})");
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

fn next_tile(x: &mut i32, y: &mut i32, stride: i32) {
    *x += 1;
    if *x >= stride {
        *x = 0;
        *y += 1;
    }
}

/// Advances looping animations on all tile and object sprites that carry `AnimFrames`.
///
/// Updates both the sprite texture/rect and the transform XY so that per-frame
/// differences in dest_x/dest_y or sprite x_off/y_off don't cause visible jitter.
/// Timer duration is kept in sync with `State::anim_interval_ms` each frame.
pub fn animate_sprites(
    time: Res<Time>,
    state: Res<State>,
    mut query: Query<(&mut AnimFrames, &mut Sprite, &mut Transform)>,
) {
    let target = Duration::from_secs_f32(state.anim_interval_ms / 1000.0);
    for (mut anim, mut sprite, mut transform) in query.iter_mut() {
        if anim.timer.duration() != target {
            anim.timer.set_duration(target);
        }
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.frame_idx = (anim.frame_idx + 1) % anim.frames.len();
            let (handle, rect, xy) = &anim.frames[anim.frame_idx];
            sprite.image = handle.clone();
            sprite.rect = Some(*rect);
            transform.translation.x = xy.x;
            transform.translation.y = xy.y;
        }
    }
}

pub fn change_map(
    mut events: MessageReader<MapChangeEvent>,
    mut commands: Commands,
    mut state: ResMut<State>,
    mut map_index: ResMut<crate::resources::map_index::MapIndex>,
    mut spawn_state: ResMut<SpawnState>,
    mut collision_grid: ResMut<CollisionGrid>,
    asset_server: Res<AssetServer>,
    tile_q: Query<Entity, With<TileSprite>>,
    obj_q: Query<Entity, With<ObjectSprite>>,
) {
    let Some(event) = events.read().last() else { return };
    let map_number = event.map_number;

    for entity in tile_q.iter().chain(obj_q.iter()) {
        commands.entity(entity).despawn();
    }

    let handle = map_index.load_or_get(map_number, &asset_server);
    state.data.maps.clear();
    state.data.maps.insert(map_number, handle);
    state.current_map = map_number;

    spawn_state.reset();
    *collision_grid = CollisionGrid::default();
    info!("[change_map] switching to map {map_number:05}");
}

pub fn map_cycle_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State>,
    mut events: MessageWriter<MapChangeEvent>,
) {
    let prev = keys.just_pressed(KeyCode::PageUp) || keys.just_pressed(KeyCode::BracketLeft);
    let next = keys.just_pressed(KeyCode::PageDown) || keys.just_pressed(KeyCode::BracketRight);
    if prev {
        events.write(MapChangeEvent { map_number: state.current_map.saturating_sub(1) });
    }
    if next {
        events.write(MapChangeEvent { map_number: (state.current_map + 1).min(MAX_MAP) });
    }
}
