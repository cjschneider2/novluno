use std::time::Duration;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::scenes::state::{
    AccountData, CleanupOnExit, SceneEntity, SceneState, CHARACTER_CLASSES,
};
use crate::systems::player::Player;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnteringGameState>()
            .add_systems(OnEnter(SceneState::EnteringGame), setup_entering_game)
            .add_systems(
                Update,
                (update_entering_game, render_entering_game_ui)
                    .run_if(in_state(SceneState::EnteringGame)),
            )
            .add_systems(OnExit(SceneState::EnteringGame), cleanup_entering_game)
            .add_systems(OnEnter(SceneState::InGame), setup_in_game_session)
            .add_systems(
                Update,
                (in_game_input_handler, render_in_game_hud)
                    .run_if(in_state(SceneState::InGame)),
            )
            .add_systems(OnExit(SceneState::InGame), cleanup_in_game_session);
    }
}

#[derive(Resource)]
pub struct EnteringGameState {
    pub timer: Timer,
    pub anim_frame: usize,
    pub anim_timer: Timer,
}

impl Default for EnteringGameState {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(1000), TimerMode::Once),
            anim_frame: 0,
            anim_timer: Timer::new(Duration::from_millis(60), TimerMode::Repeating),
        }
    }
}

fn setup_entering_game(
    mut commands: Commands,
    mut entering: ResMut<EnteringGameState>,
) {
    entering.timer.reset();
    entering.anim_frame = 0;
    entering.anim_timer.reset();

    commands.spawn((
        SceneEntity,
        CleanupOnExit(SceneState::EnteringGame),
        Name::new("EnteringGameRoot"),
    ));
}

fn cleanup_entering_game(
    mut commands: Commands,
    query: Query<Entity, With<CleanupOnExit<SceneState>>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_entering_game(
    time: Res<Time>,
    mut entering: ResMut<EnteringGameState>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    entering.timer.tick(time.delta());
    entering.anim_timer.tick(time.delta());

    if entering.anim_timer.just_finished() {
        entering.anim_frame = (entering.anim_frame + 1) % 32;
    }

    if entering.timer.is_finished() {
        next_state.set(SceneState::InGame);
    }
}

fn render_entering_game_ui(
    mut contexts: EguiContexts,
    entering: Res<EnteringGameState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    egui::Area::new(egui::Id::new("entering_game_area"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            let screen_rect = ui.max_rect();
            let center = screen_rect.center();

            ui.painter().rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_rgb(10, 10, 15),
            );

            // Animated portal / spinner simulation (32 frames)
            let angle = (entering.anim_frame as f32 / 32.0) * std::f32::consts::TAU;
            let radius = 40.0;
            let particle_pos = egui::pos2(
                center.x + angle.cos() * radius,
                center.y + angle.sin() * radius,
            );

            ui.painter().circle_filled(
                center,
                radius + 15.0,
                egui::Color32::from_rgba_unmultiplied(40, 80, 160, 60),
            );
            ui.painter().circle_filled(
                particle_pos,
                8.0,
                egui::Color32::from_rgb(120, 210, 255),
            );
        });

    egui::Window::new("entering_game_text")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 70.0])
        .title_bar(false)
        .resizable(false)
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("ENTERING GAME WORLD...")
                        .color(egui::Color32::from_rgb(240, 225, 100))
                        .size(20.0)
                        .strong(),
                );
            });
        });
}

fn setup_in_game_session(
    account: Res<AccountData>,
    mut player_q: Query<&mut Player>,
) {
    if let Some(idx) = account.selected_character_index {
        if let Some(ch) = account.characters.get(idx) {
            for mut player in player_q.iter_mut() {
                player.char_enum = ch.class;
            }
        }
    }
}

fn cleanup_in_game_session(
    mut commands: Commands,
    query: Query<Entity, With<CleanupOnExit<SceneState>>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn in_game_input_handler(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(SceneState::CharacterSelect);
    }
}

fn render_in_game_hud(
    mut contexts: EguiContexts,
    account: Res<AccountData>,
    player_q: Query<(&Player, &Transform)>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    let char_info = account
        .selected_character_index
        .and_then(|idx| account.characters.get(idx));

    let (char_name, char_lvl, class_name) = if let Some(ch) = char_info {
        let name = ch.name.as_str();
        let lvl = ch.level;
        let cname = CHARACTER_CLASSES
            .iter()
            .find(|c| c.id == ch.class)
            .map(|c| c.name)
            .unwrap_or("Warrior");
        (name, lvl, cname)
    } else {
        ("Player", 1, "Warrior")
    };

    let player_coords = if let Ok((player, _)) = player_q.single() {
        format!("({}, {})", player.tile_x, player.tile_y)
    } else {
        "(-, -)".to_string()
    };

    egui::Window::new("in_game_hud_top")
        .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
        .title_bar(false)
        .resizable(false)
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let width = ui.max_rect().width().max(400.0);
            let rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(width, 32.0));
            ui.painter().rect_filled(
                rect,
                0.0,
                egui::Color32::from_rgba_unmultiplied(15, 20, 30, 210),
            );

            ui.horizontal(|ui| {
                ui.add_space(15.0);
                ui.label(
                    egui::RichText::new(format!("Character: {} (Lv. {})", char_name, char_lvl))
                        .color(egui::Color32::from_rgb(240, 220, 100))
                        .size(14.0)
                        .strong(),
                );
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!("Class: {}", class_name))
                        .color(egui::Color32::from_rgb(180, 200, 230))
                        .size(13.0),
                );
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!("Coords: {}", player_coords))
                        .color(egui::Color32::from_rgb(160, 230, 160))
                        .size(13.0)
                        .monospace(),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(15.0);
                    let logout_btn = ui.add_sized(
                        [100.0, 24.0],
                        egui::Button::new(
                            egui::RichText::new("Logout")
                                .color(egui::Color32::WHITE)
                                .size(12.0),
                        )
                        .fill(egui::Color32::from_rgb(140, 45, 45)),
                    );

                    if logout_btn.clicked() {
                        next_state.set(SceneState::CharacterSelect);
                    }
                });
            });
        });
}
