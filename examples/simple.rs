use bevy::prelude::*;
use render_to_texture::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update_render_to_texture)
        .run();
}
