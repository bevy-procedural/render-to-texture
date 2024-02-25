/// Render only once to a compressed texture with mipmaps.

use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use render_to_texture::*;

pub fn main() {
     App::new().add_plugins((DefaultPlugins, RenderToTexturePlugin))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (wait_for_texture, bevy::window::close_on_esc))
        .run();
}

fn wait_for_texture(
    mut commands: Commands,
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(image) = render_to_texture_tasks.image("default") {
        commands.spawn((MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::new(0.0, 1.0, 0.0)))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(images.add(image)),
                ..default()
            }),
            ..default()
        },));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
) {
    render_to_texture_tasks.add(
        "default".to_string(),
        512,
        512,
        true,
        &mut commands,
        &mut images,
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegularPolygon::new(100.0, 6))),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(-0.6, 0.7, 1.4),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    );
}
