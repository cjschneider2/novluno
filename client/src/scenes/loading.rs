use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::scenes::state::{CleanupOnExit, SceneEntity, SceneState};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingProgress>()
            .add_systems(OnEnter(SceneState::Loading), setup_loading_scene)
            .add_systems(
                Update,
                (update_loading_progress, render_loading_ui)
                    .run_if(in_state(SceneState::Loading)),
            )
            .add_systems(OnExit(SceneState::Loading), cleanup_loading_scene);
    }
}

#[derive(Resource)]
pub struct LoadingProgress {
    pub current: f32,
    pub max: f32,
    pub elapsed: f32,
}

impl Default for LoadingProgress {
    fn default() -> Self {
        Self {
            current: 0.0,
            max: 100.0,
            elapsed: 0.0,
        }
    }
}

fn setup_loading_scene(mut commands: Commands, mut progress: ResMut<LoadingProgress>) {
    progress.current = 0.0;
    progress.elapsed = 0.0;
    commands.spawn((
        SceneEntity,
        CleanupOnExit(SceneState::Loading),
        Name::new("LoadingSceneRoot"),
    ));
}

fn cleanup_loading_scene(mut commands: Commands, query: Query<Entity, With<CleanupOnExit<SceneState>>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_loading_progress(
    time: Res<Time>,
    mut progress: ResMut<LoadingProgress>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    progress.elapsed += time.delta_secs();
    // Smooth progress simulation (reaching 100% in ~1.2 seconds)
    progress.current = (progress.current + time.delta_secs() * 85.0).min(progress.max);

    if progress.current >= progress.max && progress.elapsed >= 1.2 {
        next_state.set(SceneState::Login);
    }
}

fn render_loading_ui(
    mut contexts: EguiContexts,
    progress: Res<LoadingProgress>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    egui::Area::new(egui::Id::new("loading_screen_area"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            let screen_rect = ui.max_rect();
            let center = screen_rect.center();

            // Background full screen fill
            ui.painter().rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_rgb(10, 10, 15),
            );

            // Background border frame
            let panel_rect = egui::Rect::from_center_size(center, egui::vec2(480.0, 220.0));
            ui.painter().rect_filled(
                panel_rect,
                8.0,
                egui::Color32::from_rgb(20, 24, 34),
            );
            ui.painter().rect_stroke(
                panel_rect,
                8.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(70, 85, 115)),
                egui::StrokeKind::Outside,
            );

            // Progress Bar
            let ratio = (progress.current / progress.max).clamp(0.0, 1.0);
            let bar_rect = egui::Rect::from_center_size(
                egui::pos2(center.x, center.y + 20.0),
                egui::vec2(380.0, 22.0),
            );

            // Track
            ui.painter().rect_filled(
                bar_rect,
                4.0,
                egui::Color32::from_rgb(10, 12, 18),
            );
            ui.painter().rect_stroke(
                bar_rect,
                4.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 115, 140)),
                egui::StrokeKind::Outside,
            );

            // Fill
            let fill_w = bar_rect.width() * ratio;
            if fill_w > 0.0 {
                let fill_rect = egui::Rect::from_min_size(
                    bar_rect.min,
                    egui::vec2(fill_w, bar_rect.height()),
                );
                let fill_color = if ratio >= 1.0 {
                    egui::Color32::from_rgb(100, 220, 120)
                } else {
                    egui::Color32::from_rgb(60, 140, 230)
                };
                ui.painter().rect_filled(fill_rect, 4.0, fill_color);
            }
        });

    egui::Window::new("loading_window")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, -10.0])
        .title_bar(false)
        .resizable(false)
        .fixed_size([460.0, 200.0])
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(
                    egui::RichText::new("NOVLUNO ONLINE")
                        .color(egui::Color32::from_rgb(230, 215, 100))
                        .size(24.0)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new("Loading Interface & Art Assets...")
                        .color(egui::Color32::from_rgb(180, 190, 210))
                        .size(14.0),
                );

                ui.add_space(75.0);

                // Progress Percentage Text
                let ratio = (progress.current / progress.max).clamp(0.0, 1.0);
                ui.label(
                    egui::RichText::new(format!("{:.0}%", ratio * 100.0))
                        .color(egui::Color32::from_rgb(220, 225, 235))
                        .size(13.0)
                        .monospace(),
                );
            });
        });
}
