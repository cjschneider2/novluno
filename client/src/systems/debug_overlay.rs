use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

use crate::asset_loader::map_assset_loader::MapAsset;
use crate::constants::{MAX_MAP, TILE_H, TILE_W};
use crate::state::State;
use crate::systems::camera::MapCamera;
use crate::systems::collision::CollisionOverlaySettings;
use crate::systems::map_spawn::{DebugInfo, MapChangeEvent};
use crate::systems::player::{CharDebug, CharDebugColorBy, CharSpawnState, CharSprite, Player};

fn mouse_world_pos(
    window_q: &Query<&Window, With<PrimaryWindow>>,
    camera_q: &Query<(&Camera, &GlobalTransform), With<MapCamera>>,
) -> Option<Vec2> {
    let window = window_q.single().ok()?;
    let cursor = window.cursor_position()?;
    let (camera, cam_tf) = camera_q.single().ok()?;
    camera.viewport_to_world_2d(cam_tf, cursor).ok()
}

pub fn debug_overlay(
    mut contexts: EguiContexts,
    mut state: ResMut<State>,
    maps: Res<Assets<MapAsset>>,
    time: Res<Time>,
    mut smoothed_ms: Local<f32>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MapCamera>>,
    debug_q: Query<(&Transform, &DebugInfo)>,
    mut map_events: MessageWriter<MapChangeEvent>,
    mut player_q: Query<&mut Player>,
    mut char_spawn_state: ResMut<CharSpawnState>,
    mut char_debug: ResMut<CharDebug>,
    char_sprite_q: Query<&CharSprite>,
    mut collision_settings: ResMut<CollisionOverlaySettings>,
) -> Result {
    let dt_ms = time.delta_secs() * 1000.0;
    *smoothed_ms = if *smoothed_ms == 0.0 { dt_ms } else { *smoothed_ms * 0.9 + dt_ms * 0.1 };
    let fps = 1000.0 / *smoothed_ms;

    let map_name = state
        .data
        .maps
        .values()
        .next()
        .and_then(|h| maps.get(h.id()))
        .map(|m| m.0.name.clone())
        .unwrap_or_else(|| "loading…".to_string());

    let mouse_world = mouse_world_pos(&window_q, &camera_q);

    let ctx = contexts.ctx_mut()?;

    egui::Window::new("debug_overlay")
        .anchor(egui::Align2::LEFT_TOP, [4.0, 4.0])
        .title_bar(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(format!("{fps:.1} fps  {:.2} ms/frame", *smoothed_ms));
            ui.horizontal(|ui| {
                if ui.button("◀").clicked() && state.current_map > 0 {
                    map_events.write(MapChangeEvent { map_number: state.current_map.saturating_sub(1) });
                }
                if ui.button("▶").clicked() && state.current_map < MAX_MAP {
                    map_events.write(MapChangeEvent { map_number: (state.current_map + 1).min(MAX_MAP) });
                }
                ui.label(format!("Map {:05}: {map_name}", state.current_map));
            });
            ui.collapsing("Frame", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Anim interval:");
                    ui.add(
                        egui::Slider::new(&mut state.anim_interval_ms, 16.0..=500.0)
                            .suffix(" ms")
                            .fixed_decimals(0),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Frame limit:");
                    ui.add(
                        egui::Slider::new(&mut state.frame_limit_fps, 30.0..=120.0)
                            .suffix(" fps")
                            .fixed_decimals(0),
                    );
                });
            });

            // ── Player section ─────────────────────────────────────────────
            if let Ok(mut player) = player_q.single_mut() {
                ui.collapsing("Player", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Char type:");
                        let prev = player.char_enum;
                        ui.add(egui::DragValue::new(&mut player.char_enum).range(0..=9));
                        if player.char_enum != prev {
                            // Trigger full asset reload for the new char type.
                            char_spawn_state.force_reload();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Anim base:");
                        let prev = player.action.anim_base() as u32;
                        let mut base = prev;
                        ui.add(egui::DragValue::new(&mut base).range(0..=1800_u32));
                        if base != prev {
                            // Free-form override stored as a raw anim_base via the timer field.
                            // We re-use frame_idx and mark dirty; the enum stays as-is.
                            player.frame_idx = 0;
                            char_spawn_state.dirty = true;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Direction:");
                        ui.label(format!("{} ({})", player.direction.name(), player.direction.as_usize()));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Action:");
                        ui.label(player.action.name());
                    });
                    ui.label(format!("Frame: {}", player.frame_idx));
                    ui.label(format!(
                        "Tile: ({}, {})  sub={}",
                        player.tile_x, player.tile_y, player.tile_pos
                    ));
                    ui.separator();
                    ui.checkbox(&mut char_debug.show_layers, "Debug layer colors");
                    if char_debug.show_layers {
                        ui.horizontal(|ui| {
                            ui.label("Color by:");
                            ui.selectable_value(&mut char_debug.color_by, CharDebugColorBy::Layer,    "layer");
                            ui.selectable_value(&mut char_debug.color_by, CharDebugColorBy::DrawType, "draw_type");
                            ui.selectable_value(&mut char_debug.color_by, CharDebugColorBy::RenderZ,  "render_z");
                        });
                        if char_debug.color_by == CharDebugColorBy::Layer {
                            for (v, name, r, g, b) in [
                                (0i32, "body",       255u8, 77u8,  77u8),
                                (1,    "skill FX",   77,    255,   77),
                                (2,    "shadow",     77,    77,    255),
                                (3,    "clothes",    255,   255,   77),
                                (4,    "hair",       77,    255,   255),
                                (5,    "weapon FX",  255,   77,    255),
                                (6,    "weapon FX2", 255,   153,   0),
                            ] {
                                ui.colored_label(egui::Color32::from_rgb(r, g, b),
                                    format!("[{v}] {name}"));
                            }
                        }
                        ui.separator();
                        let sprites: Vec<_> = char_sprite_q.iter().collect();
                        ui.label(format!("{} sub-sprites:", sprites.len()));
                        for cs in &sprites {
                            ui.label(format!(
                                "  layer={} dt=0x{:X} rz={}",
                                cs.layer, cs.draw_type, cs.render_z
                            ));
                        }
                    }
                });
            }

            // ── Map section ────────────────────────────────────────────────
            ui.collapsing("Map", |ui| {
                ui.checkbox(&mut collision_settings.show, "Show collision overlay");
            });

            match mouse_world {
                Some(pos) => {
                    let tile_x = (pos.x / TILE_W).floor() as i32;
                    let tile_y = (-pos.y / TILE_H).floor() as i32;
                    ui.label(format!("Mouse: ({:.0}, {:.0})", pos.x, pos.y));
                    ui.label(format!("Tile:  ({tile_x}, {tile_y})"));

                    let mut sprites: Vec<(f32, &str)> = debug_q
                        .iter()
                        .filter(|(_, info)| info.tile_x == tile_x && info.tile_y == tile_y)
                        .map(|(_, info)| (info.z, info.kind))
                        .collect();
                    sprites.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                    if !sprites.is_empty() {
                        ui.separator();
                        ui.label(format!("{} sprites at tile:", sprites.len()));
                        for (z, kind) in &sprites {
                            ui.label(format!("  [{kind}] z={z:.4}"));
                        }
                    }
                }
                None => {
                    ui.label("Mouse: --");
                }
            }
        });

    Ok(())
}

/// Draws bounding box gizmos for every sprite belonging to the tile under
/// the mouse cursor. Tiles = yellow, objects = cyan.
pub fn draw_debug_gizmos(
    mut gizmos: Gizmos,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MapCamera>>,
    debug_q: Query<(&Transform, &DebugInfo)>,
) {
    let Some(mouse_world) = mouse_world_pos(&window_q, &camera_q) else {
        return;
    };

    let tile_x = (mouse_world.x / TILE_W).floor() as i32;
    let tile_y = (-mouse_world.y / TILE_H).floor() as i32;

    let cell_center = Vec2::new(
        tile_x as f32 * TILE_W + TILE_W * 0.5,
        -(tile_y as f32 * TILE_H) - TILE_H * 0.5,
    );
    gizmos.rect_2d(
        Isometry2d::from_translation(cell_center),
        Vec2::new(TILE_W, TILE_H),
        Color::WHITE,
    );

    for (transform, info) in debug_q.iter() {
        if info.tile_x != tile_x || info.tile_y != tile_y {
            continue;
        }

        let center = Vec2::new(
            transform.translation.x + info.render_w * 0.5,
            transform.translation.y - info.render_h * 0.5,
        );
        let color = if info.kind == "tile" {
            Color::srgb(1.0, 1.0, 0.0)
        } else {
            Color::srgb(0.0, 1.0, 1.0)
        };
        gizmos.rect_2d(
            Isometry2d::from_translation(center),
            Vec2::new(info.render_w, info.render_h),
            color,
        );
    }
}
