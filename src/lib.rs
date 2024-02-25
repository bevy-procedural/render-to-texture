use bevy::prelude::*;
pub use render::{create_render_texture, RenderToTextureTasks};
mod gpu2cpu;
mod render;
mod compress;

pub struct RenderToTexturePlugin;

impl Plugin for RenderToTexturePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RenderToTextureTasks>()
            .insert_resource(RenderToTextureTasks::default())
            .add_plugins(gpu2cpu::ImageExportPlugin::default())
            .add_systems(PreUpdate, render::update_render_to_texture);
    }
}
