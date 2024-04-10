use bevy::prelude::*;

#[derive(Component)]
pub struct DebugOriginMarker;

pub fn spawn_debug_marker(mut commands: Commands) {
    // Bright magenta 10×10 square at world origin (0,0).
    // Tile (0,0) should render at exactly this position; remove once validated.
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 1.0), Vec2::splat(10.0)),
        Transform::from_xyz(0.0, 0.0, 200.0),
        DebugOriginMarker,
    ));
}
