use std::time::Duration;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::scenes::state::{AccountData, CleanupOnExit, SceneEntity, SceneState};

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoginAnimationState>()
            .init_resource::<LoginModalState>()
            .add_systems(OnEnter(SceneState::Login), setup_login_scene)
            .add_systems(
                Update,
                (update_sun_animation, render_login_ui)
                    .run_if(in_state(SceneState::Login)),
            )
            .add_systems(OnExit(SceneState::Login), cleanup_login_scene);
    }
}

#[derive(Resource)]
pub struct LoginAnimationState {
    pub sun_frame: usize,
    pub sun_timer: Timer,
    pub is_solo: bool,
}

impl Default for LoginAnimationState {
    fn default() -> Self {
        Self {
            sun_frame: 0,
            sun_timer: Timer::new(Duration::from_millis(110), TimerMode::Repeating),
            is_solo: false,
        }
    }
}

#[derive(Resource)]
pub struct LoginModalState {
    pub is_open: bool,
    pub username: String,
    pub password: String,
    pub error_msg: Option<String>,
}

impl Default for LoginModalState {
    fn default() -> Self {
        Self {
            is_open: false,
            username: "Player1".to_string(),
            password: "password".to_string(),
            error_msg: None,
        }
    }
}

fn setup_login_scene(
    mut commands: Commands,
    mut anim: ResMut<LoginAnimationState>,
    mut modal: ResMut<LoginModalState>,
) {
    anim.sun_frame = 0;
    anim.is_solo = false;
    modal.is_open = false;
    modal.error_msg = None;

    commands.spawn((
        SceneEntity,
        CleanupOnExit(SceneState::Login),
        Name::new("LoginSceneRoot"),
    ));
}

fn cleanup_login_scene(mut commands: Commands, query: Query<Entity, With<CleanupOnExit<SceneState>>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_sun_animation(time: Res<Time>, mut anim: ResMut<LoginAnimationState>) {
    anim.sun_timer.tick(time.delta());
    if anim.sun_timer.just_finished() {
        if !anim.is_solo {
            // 9 frames for initial rising sun
            anim.sun_frame += 1;
            if anim.sun_frame >= 9 {
                anim.sun_frame = 0;
                anim.is_solo = true;
                anim.sun_timer = Timer::new(Duration::from_millis(180), TimerMode::Repeating);
            }
        } else {
            // 5 frames for solo glowing sun loop
            anim.sun_frame = (anim.sun_frame + 1) % 5;
        }
    }
}

fn render_login_ui(
    mut contexts: EguiContexts,
    anim: Res<LoginAnimationState>,
    mut modal: ResMut<LoginModalState>,
    mut account: ResMut<AccountData>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    egui::Area::new(egui::Id::new("login_background_area"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            let screen_rect = ui.max_rect();
            let center = screen_rect.center();

            // Full screen background
            ui.painter().rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_rgb(12, 14, 20),
            );

            // Border frame (800x600 centered)
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

            // Sun Animation Canvas in upper center
            let sun_center = egui::pos2(center.x, center.y - 80.0);
            let sun_radius = 45.0 + (anim.sun_frame as f32 * 3.5);
            let sun_glow_alpha = if anim.is_solo {
                160 + (anim.sun_frame as u8 * 18)
            } else {
                100 + (anim.sun_frame as u8 * 14)
            };

            // Glowing Sun rays and sphere
            ui.painter().circle_filled(
                sun_center,
                sun_radius + 15.0,
                egui::Color32::from_rgba_unmultiplied(235, 120, 40, sun_glow_alpha / 2),
            );
            ui.painter().circle_filled(
                sun_center,
                sun_radius,
                egui::Color32::from_rgba_unmultiplied(255, 180, 50, sun_glow_alpha),
            );
            ui.painter().circle_filled(
                sun_center,
                sun_radius * 0.7,
                egui::Color32::from_rgb(255, 235, 150),
            );
        });

    egui::Window::new("login_ui_window")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .title_bar(false)
        .resizable(false)
        .fixed_size([780.0, 580.0])
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Ver: 1.0 (Novluno Client)")
                        .color(egui::Color32::from_rgb(200, 210, 225))
                        .size(13.0)
                        .monospace(),
                );
            });

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(
                    egui::RichText::new("REDMOON : NOVLUNO")
                        .color(egui::Color32::from_rgb(245, 210, 90))
                        .size(36.0)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new("The Classic Sci-Fi MMORPG Reborn")
                        .color(egui::Color32::from_rgb(170, 185, 215))
                        .size(15.0),
                );

                ui.add_space(340.0);

                // Login & New ID Action Buttons
                ui.horizontal(|ui| {
                    let total_btn_w = 280.0;
                    ui.add_space((ui.available_width() - total_btn_w) / 2.0);

                    let login_btn = ui.add_sized(
                        [130.0, 40.0],
                        egui::Button::new(
                            egui::RichText::new("LOGIN")
                                .color(egui::Color32::WHITE)
                                .size(16.0)
                                .strong(),
                        )
                        .fill(egui::Color32::from_rgb(45, 110, 210)),
                    );

                    if login_btn.clicked() {
                        modal.is_open = true;
                    }

                    ui.add_space(20.0);

                    let new_id_btn = ui.add_sized(
                        [130.0, 40.0],
                        egui::Button::new(
                            egui::RichText::new("NEW ID")
                                .color(egui::Color32::WHITE)
                                .size(16.0)
                                .strong(),
                        )
                        .fill(egui::Color32::from_rgb(70, 85, 105)),
                    );

                    if new_id_btn.clicked() {
                        modal.is_open = true;
                    }
                });
            });
        });

    // Login Modal
    if modal.is_open {
        egui::Window::new("login_modal")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .title_bar(false)
            .resizable(false)
            .fixed_size([360.0, 240.0])
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                ui.painter().rect_filled(
                    rect,
                    8.0,
                    egui::Color32::from_rgb(24, 28, 40),
                );
                ui.painter().rect_stroke(
                    rect,
                    8.0,
                    egui::Stroke::new(2.0, egui::Color32::from_rgb(90, 120, 175)),
                    egui::StrokeKind::Outside,
                );

                ui.vertical_centered(|ui| {
                    ui.add_space(16.0);
                    ui.heading(
                        egui::RichText::new("ACCOUNT LOGIN")
                            .color(egui::Color32::from_rgb(240, 215, 100))
                            .size(20.0)
                            .strong(),
                    );
                    ui.add_space(16.0);

                    egui::Grid::new("login_grid")
                        .num_columns(2)
                        .spacing([12.0, 12.0])
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("Username:")
                                    .color(egui::Color32::from_rgb(210, 220, 235))
                                    .size(14.0),
                            );
                            ui.text_edit_singleline(&mut modal.username);
                            ui.end_row();

                            ui.label(
                                egui::RichText::new("Password:")
                                    .color(egui::Color32::from_rgb(210, 220, 235))
                                    .size(14.0),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut modal.password)
                                    .password(true),
                            );
                            ui.end_row();
                        });

                    if let Some(ref err) = modal.error_msg {
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(err)
                                .color(egui::Color32::from_rgb(240, 80, 80))
                                .size(12.0),
                        );
                    }

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        let total_modal_btn_w = 240.0;
                        ui.add_space((ui.available_width() - total_modal_btn_w) / 2.0);

                        let ok_btn = ui.add_sized(
                            [110.0, 32.0],
                            egui::Button::new(
                                egui::RichText::new("OK")
                                    .color(egui::Color32::WHITE)
                                    .size(14.0)
                                    .strong(),
                            )
                            .fill(egui::Color32::from_rgb(45, 130, 220)),
                        );

                        if ok_btn.clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            if modal.username.trim().is_empty() {
                                modal.error_msg = Some("Please enter a username".to_string());
                            } else {
                                account.username = modal.username.trim().to_string();
                                account.logged_in = true;
                                modal.is_open = false;
                                next_state.set(SceneState::CharacterSelect);
                            }
                        }

                        ui.add_space(20.0);

                        let cancel_btn = ui.add_sized(
                            [110.0, 32.0],
                            egui::Button::new(
                                egui::RichText::new("Cancel")
                                    .color(egui::Color32::WHITE)
                                    .size(14.0),
                            )
                            .fill(egui::Color32::from_rgb(80, 90, 110)),
                        );

                        if cancel_btn.clicked() {
                            modal.is_open = false;
                            modal.error_msg = None;
                        }
                    });
                });
            });
    }
}
