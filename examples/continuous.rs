/// Render each frame to a texture that is directly bound to a material.

use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use render_to_texture::*;

pub fn main() {
    App::new().add_plugins((DefaultPlugins, RenderToTexturePlugin))
        .add_systems(Startup, setup_scene)
        .add_systems(Update,  bevy::window::close_on_esc)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let (image_handle, _) = create_render_texture(512, 512, &mut commands, &mut images, 1, true);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegularPolygon::new(100.0, 6))),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(-0.6, 0.7, 1.4),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::new(0.0, 1.0, 0.0)))),
        material: standard_materials.add(StandardMaterial {
            base_color_texture: Some(image_handle),
            ..default()
        }),
        ..default()
    },));

    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    );
}
