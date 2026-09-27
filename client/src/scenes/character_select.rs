use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::scenes::state::{
    AccountData, CleanupOnExit, SceneEntity, SceneState, CHARACTER_CLASSES, COLOR_PALETTE,
};

pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::CharacterSelect), setup_char_select_scene)
            .add_systems(
                Update,
                render_char_select_ui.run_if(in_state(SceneState::CharacterSelect)),
            )
            .add_systems(OnExit(SceneState::CharacterSelect), cleanup_char_select_scene);
    }
}

fn setup_char_select_scene(mut commands: Commands, mut account: ResMut<AccountData>) {
    if !account.characters.is_empty() && account.selected_character_index.is_none() {
        account.selected_character_index = Some(0);
    }

    commands.spawn((
        SceneEntity,
        CleanupOnExit(SceneState::CharacterSelect),
        Name::new("CharacterSelectSceneRoot"),
    ));
}

fn cleanup_char_select_scene(
    mut commands: Commands,
    query: Query<Entity, With<CleanupOnExit<SceneState>>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn render_char_select_ui(
    mut contexts: EguiContexts,
    mut account: ResMut<AccountData>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    egui::Area::new(egui::Id::new("char_select_bg"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            let screen_rect = ui.max_rect();
            let center = screen_rect.center();

            ui.painter().rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_rgb(12, 14, 20),
            );

            let frame_w = 800.0f32.min(screen_rect.width() - 20.0);
            let frame_h = 600.0f32.min(screen_rect.height() - 20.0);
            let frame_rect = egui::Rect::from_center_size(center, egui::vec2(frame_w, frame_h));

            ui.painter().rect_filled(
                frame_rect,
                10.0,
                egui::Color32::from_rgb(18, 22, 32),
            );
            ui.painter().rect_stroke(
                frame_rect,
                10.0,
                egui::Stroke::new(2.5, egui::Color32::from_rgb(85, 105, 145)),
                egui::StrokeKind::Outside,
            );
        });

    egui::Window::new("char_select_window")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .title_bar(false)
        .resizable(false)
        .fixed_size([780.0, 580.0])
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(
                    egui::RichText::new("SELECT CHARACTER")
                        .color(egui::Color32::from_rgb(245, 215, 90))
                        .size(28.0)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new(format!("Account: {}", account.username))
                        .color(egui::Color32::from_rgb(175, 190, 215))
                        .size(14.0),
                );

                ui.add_space(20.0);

                // 3 Character Slots Row
                ui.horizontal(|ui| {
                    let slot_w = 210.0;
                    let slot_h = 300.0;
                    let total_slots_w = 3.0 * slot_w + 2.0 * 20.0;
                    ui.add_space((ui.available_width() - total_slots_w) / 2.0);

                    for slot_idx in 0..3 {
                        let is_selected = account.selected_character_index == Some(slot_idx);
                        let char_opt = account.characters.get(slot_idx).cloned();

                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(slot_w, slot_h),
                            egui::Sense::click(),
                        );

                        let bg_color = if is_selected {
                            egui::Color32::from_rgb(28, 42, 65)
                        } else if response.hovered() {
                            egui::Color32::from_rgb(22, 28, 42)
                        } else {
                            egui::Color32::from_rgb(15, 18, 28)
                        };

                        let border_stroke = if is_selected {
                            egui::Stroke::new(2.5, egui::Color32::from_rgb(240, 200, 80))
                        } else {
                            egui::Stroke::new(1.5, egui::Color32::from_rgb(60, 75, 105))
                        };

                        ui.painter().rect_filled(rect, 8.0, bg_color);
                        ui.painter().rect_stroke(rect, 8.0, border_stroke, egui::StrokeKind::Outside);

                        if let Some(ch) = char_opt {
                            if response.clicked() {
                                account.selected_character_index = Some(slot_idx);
                            }
                            if response.double_clicked() {
                                account.selected_character_index = Some(slot_idx);
                                next_state.set(SceneState::EnteringGame);
                            }

                            // Avatar preview box
                            let avatar_rect = egui::Rect::from_center_size(
                                egui::pos2(rect.center().x, rect.min.y + 100.0),
                                egui::vec2(130.0, 140.0),
                            );
                            ui.painter().rect_filled(
                                avatar_rect,
                                6.0,
                                egui::Color32::from_rgb(25, 30, 45),
                            );

                            let class_info = CHARACTER_CLASSES
                                .iter()
                                .find(|c| c.id == ch.class)
                                .map(|c| c.name)
                                .unwrap_or("Unknown");

                            let head_center = egui::pos2(avatar_rect.center().x, avatar_rect.min.y + 40.0);
                            let hair_col = COLOR_PALETTE[ch.hair_color % 30];
                            let body_col = COLOR_PALETTE[ch.body_color % 30];

                            // Hair silhouette
                            ui.painter().circle_filled(
                                head_center,
                                24.0,
                                egui::Color32::from_rgb(hair_col[0], hair_col[1], hair_col[2]),
                            );
                            // Body silhouette
                            let torso_rect = egui::Rect::from_center_size(
                                egui::pos2(avatar_rect.center().x, avatar_rect.min.y + 90.0),
                                egui::vec2(60.0, 50.0),
                            );
                            ui.painter().rect_filled(
                                torso_rect,
                                6.0,
                                egui::Color32::from_rgb(body_col[0], body_col[1], body_col[2]),
                            );

                            // Text details painted directly inside slot rect
                            ui.painter().text(
                                egui::pos2(rect.center().x, rect.min.y + 200.0),
                                egui::Align2::CENTER_CENTER,
                                &ch.name,
                                egui::FontId::proportional(16.0),
                                egui::Color32::from_rgb(240, 220, 100),
                            );
                            ui.painter().text(
                                egui::pos2(rect.center().x, rect.min.y + 225.0),
                                egui::Align2::CENTER_CENTER,
                                format!("Level {}", ch.level),
                                egui::FontId::proportional(14.0),
                                egui::Color32::from_rgb(180, 230, 180),
                            );
                            ui.painter().text(
                                egui::pos2(rect.center().x, rect.min.y + 250.0),
                                egui::Align2::CENTER_CENTER,
                                class_info,
                                egui::FontId::proportional(13.0),
                                egui::Color32::from_rgb(160, 180, 210),
                            );
                        } else {
                            if response.clicked() {
                                next_state.set(SceneState::CharacterCreate);
                            }

                            ui.painter().text(
                                egui::pos2(rect.center().x, rect.min.y + 110.0),
                                egui::Align2::CENTER_CENTER,
                                "+",
                                egui::FontId::proportional(36.0),
                                egui::Color32::from_rgb(100, 120, 150),
                            );
                            ui.painter().text(
                                egui::pos2(rect.center().x, rect.min.y + 155.0),
                                egui::Align2::CENTER_CENTER,
                                "Empty Slot\nClick to Create",
                                egui::FontId::proportional(14.0),
                                egui::Color32::from_rgb(140, 155, 180),
                            );
                        }

                        if slot_idx < 2 {
                            ui.add_space(20.0);
                        }
                    }
                });

                ui.add_space(30.0);

                // Action Buttons Row (Start Game, Add Character, Delete, Logout)
                ui.horizontal(|ui| {
                    let btn_w = 140.0;
                    let btn_h = 36.0;
                    let spacing = 15.0;
                    let total_w = 4.0 * btn_w + 3.0 * spacing;
                    ui.add_space((ui.available_width() - total_w) / 2.0);

                    let has_selected = account.selected_character_index.is_some()
                        && account
                            .selected_character_index
                            .and_then(|i| account.characters.get(i))
                            .is_some();

                    let start_btn = ui.add_enabled(
                        has_selected,
                        egui::Button::new(
                            egui::RichText::new("Start Game")
                                .color(egui::Color32::WHITE)
                                .size(14.0)
                                .strong(),
                        )
                        .min_size(egui::vec2(btn_w, btn_h))
                        .fill(egui::Color32::from_rgb(45, 130, 220)),
                    );

                    if start_btn.clicked() {
                        next_state.set(SceneState::EnteringGame);
                    }

                    ui.add_space(spacing);

                    let can_add = account.characters.len() < 3;
                    let add_btn = ui.add_enabled(
                        can_add,
                        egui::Button::new(
                            egui::RichText::new("Add Character")
                                .color(egui::Color32::WHITE)
                                .size(14.0),
                        )
                        .min_size(egui::vec2(btn_w, btn_h))
                        .fill(egui::Color32::from_rgb(50, 140, 90)),
                    );

                    if add_btn.clicked() {
                        next_state.set(SceneState::CharacterCreate);
                    }

                    ui.add_space(spacing);

                    let delete_btn = ui.add_enabled(
                        has_selected,
                        egui::Button::new(
                            egui::RichText::new("Delete")
                                .color(egui::Color32::WHITE)
                                .size(14.0),
                        )
                        .min_size(egui::vec2(btn_w, btn_h))
                        .fill(egui::Color32::from_rgb(160, 50, 50)),
                    );

                    if delete_btn.clicked() {
                        if let Some(idx) = account.selected_character_index {
                            if idx < account.characters.len() {
                                account.characters.remove(idx);
                                account.selected_character_index = if account.characters.is_empty() {
                                    None
                                } else {
                                    Some(0)
                                };
                            }
                        }
                    }

                    ui.add_space(spacing);

                    let logout_btn = ui.add_sized(
                        [btn_w, btn_h],
                        egui::Button::new(
                            egui::RichText::new("Logout")
                                .color(egui::Color32::WHITE)
                                .size(14.0),
                        )
                        .fill(egui::Color32::from_rgb(70, 80, 100)),
                    );

                    if logout_btn.clicked() {
                        account.logged_in = false;
                        next_state.set(SceneState::Login);
                    }
                });
            });
        });
}
