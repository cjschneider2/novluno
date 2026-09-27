use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::scenes::state::{
    AccountData, CharacterSummary, CleanupOnExit, SceneEntity, SceneState, CHARACTER_CLASSES,
    COLOR_PALETTE,
};

pub struct CharacterCreatePlugin;

impl Plugin for CharacterCreatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterCreationDraft>()
            .add_systems(OnEnter(SceneState::CharacterCreate), setup_char_create_scene)
            .add_systems(
                Update,
                render_char_create_ui.run_if(in_state(SceneState::CharacterCreate)),
            )
            .add_systems(OnExit(SceneState::CharacterCreate), cleanup_char_create_scene);
    }
}

#[derive(Resource)]
pub struct CharacterCreationDraft {
    pub name: String,
    pub class_index: u32,
    pub hair_color: usize,
    pub body_color: usize,
    pub error_msg: Option<String>,
}

impl Default for CharacterCreationDraft {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            class_index: 0,
            hair_color: 0,
            body_color: 0,
            error_msg: None,
        }
    }
}

fn setup_char_create_scene(mut commands: Commands, mut draft: ResMut<CharacterCreationDraft>) {
    draft.name = "".to_string();
    draft.class_index = 0;
    draft.hair_color = 0;
    draft.body_color = 0;
    draft.error_msg = None;

    commands.spawn((
        SceneEntity,
        CleanupOnExit(SceneState::CharacterCreate),
        Name::new("CharacterCreateSceneRoot"),
    ));
}

fn cleanup_char_create_scene(
    mut commands: Commands,
    query: Query<Entity, With<CleanupOnExit<SceneState>>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn render_char_create_ui(
    mut contexts: EguiContexts,
    mut draft: ResMut<CharacterCreationDraft>,
    mut account: ResMut<AccountData>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    egui::Area::new(egui::Id::new("char_create_bg"))
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

    egui::Window::new("char_create_window")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .title_bar(false)
        .resizable(false)
        .fixed_size([780.0, 580.0])
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.heading(
                    egui::RichText::new("CREATE NEW CHARACTER")
                        .color(egui::Color32::from_rgb(245, 215, 90))
                        .size(24.0)
                        .strong(),
                );
                ui.add_space(10.0);

                // Top Class Selector Buttons (0 to 5)
                ui.horizontal(|ui| {
                    let btn_w = 110.0;
                    let btn_h = 32.0;
                    let spacing = 8.0;
                    let total_w = 6.0 * btn_w + 5.0 * spacing;
                    ui.add_space((ui.available_width() - total_w) / 2.0);

                    for i in 0..6 {
                        let class_info = &CHARACTER_CLASSES[i];
                        let is_sel = draft.class_index == class_info.id;
                        let btn_color = if is_sel {
                            egui::Color32::from_rgb(45, 120, 210)
                        } else {
                            egui::Color32::from_rgb(25, 32, 45)
                        };

                        let btn = ui.add_sized(
                            [btn_w, btn_h],
                            egui::Button::new(
                                egui::RichText::new(class_info.name)
                                    .color(if is_sel { egui::Color32::WHITE } else { egui::Color32::from_rgb(190, 205, 225) })
                                    .size(13.0)
                                    .strong(),
                            )
                            .fill(btn_color),
                        );

                        if btn.clicked() {
                            draft.class_index = class_info.id;
                        }

                        if i < 5 {
                            ui.add_space(spacing);
                        }
                    }
                });

                ui.add_space(6.0);

                // Bottom Class Selector Buttons (6 to 8)
                ui.horizontal(|ui| {
                    let btn_w = 110.0;
                    let btn_h = 32.0;
                    let spacing = 8.0;
                    let total_w = 3.0 * btn_w + 2.0 * spacing;
                    ui.add_space((ui.available_width() - total_w) / 2.0);

                    for i in 6..9 {
                        let class_info = &CHARACTER_CLASSES[i];
                        let is_sel = draft.class_index == class_info.id;
                        let btn_color = if is_sel {
                            egui::Color32::from_rgb(45, 120, 210)
                        } else {
                            egui::Color32::from_rgb(25, 32, 45)
                        };

                        let btn = ui.add_sized(
                            [btn_w, btn_h],
                            egui::Button::new(
                                egui::RichText::new(class_info.name)
                                    .color(if is_sel { egui::Color32::WHITE } else { egui::Color32::from_rgb(190, 205, 225) })
                                    .size(13.0)
                                    .strong(),
                            )
                            .fill(btn_color),
                        );

                        if btn.clicked() {
                            draft.class_index = class_info.id;
                        }

                        if i < 8 {
                            ui.add_space(spacing);
                        }
                    }
                });

                ui.add_space(15.0);

                // Middle Area: Left Preview & Right Class Description / Name Form
                ui.horizontal(|ui| {
                    ui.add_space(40.0);

                    // Preview Box container
                    let (preview_rect, _) = ui.allocate_exact_size(
                        egui::vec2(220.0, 190.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(
                        preview_rect,
                        8.0,
                        egui::Color32::from_rgb(20, 24, 36),
                    );
                    ui.painter().rect_stroke(
                        preview_rect,
                        8.0,
                        egui::Stroke::new(1.5, egui::Color32::from_rgb(60, 75, 110)),
                        egui::StrokeKind::Outside,
                    );

                    // Render silhouette preview
                    let head_center = egui::pos2(preview_rect.center().x, preview_rect.min.y + 55.0);
                    let hair_col = COLOR_PALETTE[draft.hair_color % 30];
                    let body_col = COLOR_PALETTE[draft.body_color % 30];

                    ui.painter().circle_filled(
                        head_center,
                        28.0,
                        egui::Color32::from_rgb(hair_col[0], hair_col[1], hair_col[2]),
                    );
                    let torso_rect = egui::Rect::from_center_size(
                        egui::pos2(preview_rect.center().x, preview_rect.min.y + 120.0),
                        egui::vec2(75.0, 65.0),
                    );
                    ui.painter().rect_filled(
                        torso_rect,
                        8.0,
                        egui::Color32::from_rgb(body_col[0], body_col[1], body_col[2]),
                    );

                    ui.add_space(20.0);

                    // Right Lore & Name Input Container
                    let active_class = &CHARACTER_CLASSES[draft.class_index.min(8) as usize];
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(format!("Class: {} - {}", active_class.name, active_class.title))
                                .color(egui::Color32::from_rgb(240, 215, 100))
                                .size(16.0)
                                .strong(),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(active_class.description)
                                .color(egui::Color32::from_rgb(180, 195, 220))
                                .size(13.0),
                        );

                        ui.add_space(15.0);

                        // Name Input
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Character Name:")
                                    .color(egui::Color32::WHITE)
                                    .size(14.0)
                                    .strong(),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut draft.name)
                                    .desired_width(180.0)
                                    .hint_text("Enter name (1-14 chars)"),
                            );
                        });

                        if let Some(ref err) = draft.error_msg {
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(err)
                                    .color(egui::Color32::from_rgb(240, 80, 80))
                                    .size(12.0),
                            );
                        }
                    });
                });

                ui.add_space(15.0);

                // Color Palettes (Hair Color & Body Color)
                ui.vertical(|ui| {
                    // Hair Palette Row
                    ui.horizontal(|ui| {
                        ui.add_space(40.0);
                        ui.label(
                            egui::RichText::new("Hair Color:")
                                .color(egui::Color32::from_rgb(210, 225, 240))
                                .size(13.0),
                        );
                        ui.add_space(10.0);

                        for c_idx in 0..30 {
                            let rgb = COLOR_PALETTE[c_idx];
                            let is_sel = draft.hair_color == c_idx;
                            let (rect, resp) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::click());

                            ui.painter().rect_filled(
                                rect,
                                2.0,
                                egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]),
                            );
                            if is_sel {
                                ui.painter().rect_stroke(
                                    rect,
                                    2.0,
                                    egui::Stroke::new(2.0, egui::Color32::WHITE),
                                    egui::StrokeKind::Outside,
                                );
                            }
                            if resp.clicked() {
                                draft.hair_color = c_idx;
                            }
                            ui.add_space(4.0);
                        }
                    });

                    ui.add_space(8.0);

                    // Body Palette Row
                    ui.horizontal(|ui| {
                        ui.add_space(40.0);
                        ui.label(
                            egui::RichText::new("Body Color:")
                                .color(egui::Color32::from_rgb(210, 225, 240))
                                .size(13.0),
                        );
                        ui.add_space(7.0);

                        for c_idx in 0..30 {
                            let rgb = COLOR_PALETTE[c_idx];
                            let is_sel = draft.body_color == c_idx;
                            let (rect, resp) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::click());

                            ui.painter().rect_filled(
                                rect,
                                2.0,
                                egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]),
                            );
                            if is_sel {
                                ui.painter().rect_stroke(
                                    rect,
                                    2.0,
                                    egui::Stroke::new(2.0, egui::Color32::WHITE),
                                    egui::StrokeKind::Outside,
                                );
                            }
                            if resp.clicked() {
                                draft.body_color = c_idx;
                            }
                            ui.add_space(4.0);
                        }
                    });
                });

                ui.add_space(20.0);

                // OK / Cancel Action Buttons
                ui.horizontal(|ui| {
                    let btn_w = 130.0;
                    let btn_h = 36.0;
                    let total_w = 2.0 * btn_w + 20.0;
                    ui.add_space((ui.available_width() - total_w) / 2.0);

                    let ok_btn = ui.add_sized(
                        [btn_w, btn_h],
                        egui::Button::new(
                            egui::RichText::new("OK")
                                .color(egui::Color32::WHITE)
                                .size(15.0)
                                .strong(),
                        )
                        .fill(egui::Color32::from_rgb(45, 130, 220)),
                    );

                    if ok_btn.clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let clean_name = draft.name.trim();
                        if clean_name.is_empty() {
                            draft.error_msg = Some("Please enter a character name".to_string());
                        } else if clean_name.len() > 14 {
                            draft.error_msg = Some("Name must be 14 characters or less".to_string());
                        } else if !clean_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            draft.error_msg = Some("Name contains invalid characters".to_string());
                        } else {
                            let new_char = CharacterSummary {
                                name: clean_name.to_string(),
                                class: draft.class_index,
                                level: 1,
                                hair_color: draft.hair_color,
                                body_color: draft.body_color,
                                selected: true,
                            };

                            let new_index = account.characters.len();
                            account.characters.push(new_char);
                            account.selected_character_index = Some(new_index);

                            next_state.set(SceneState::CharacterSelect);
                        }
                    }

                    ui.add_space(20.0);

                    let cancel_btn = ui.add_sized(
                        [btn_w, btn_h],
                        egui::Button::new(
                            egui::RichText::new("Cancel")
                                .color(egui::Color32::WHITE)
                                .size(15.0),
                        )
                        .fill(egui::Color32::from_rgb(80, 90, 110)),
                    );

                    if cancel_btn.clicked() {
                        next_state.set(SceneState::CharacterSelect);
                    }
                });
            });
        });
}
