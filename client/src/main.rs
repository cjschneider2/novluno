use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};

pub struct CustomDefaultPlugin;
impl Plugin for CustomDefaultPlugin{
    // NOTE: this is needed for working around a bug in wgpu which spams the
    //       logs with errors due to a bug related to vulkan and amd drivers:
    //  SEE: https://github.com/gfx-rs/wgpu/issues/4247
    #[cfg(target_os = "windows")]
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
            ..default()
        }));
    }
    #[cfg(not(target_os = "windows"))]
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}

fn main() {
    App::new()
        .add_plugins(CustomDefaultPlugin)
        .run();
}
