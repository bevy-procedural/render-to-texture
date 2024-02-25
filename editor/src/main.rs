use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use render_to_texture::*;
use std::f32::consts::PI;

pub fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1920.0, 1080.0),
            position: WindowPosition::Centered(MonitorSelection::Index(1)),
            decorations: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(RenderToTexturePlugin)
    .add_systems(Startup, setup_scene)
    .add_systems(Update, (bevy::window::close_on_esc,))
    .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials2: ResMut<Assets<ColorMaterial>>,
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
) {
    let first_pass_layer = render_to_texture_tasks.add(512, 512, &mut commands, &mut images);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegularPolygon::new(100.0, 6))),
            material: materials2.add(Color::RED),
            transform: Transform::from_xyz(-0.6, 0.7, 1.4),
            ..default()
        },
        first_pass_layer,
    ));

    /*let (image_handle, first_pass_layer, _) =
        create_render_texture(512, 512, &mut commands, &mut images, 1);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegularPolygon::new(100.0, 6))),
            material: materials2.add(Color::RED),
            transform: Transform::from_xyz(-0.6, 0.7, 1.4),
            ..default()
        },
        first_pass_layer,
    ));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::new(0.0, 1.0, 0.0)))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(image_handle),
            ..default()
        }),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    },));*/

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::default())),
        material: materials.add(StandardMaterial { ..default() }),
        transform: Transform::from_xyz(-0.6, 0.7, 1.4),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 3.),
            ..default()
        },
        ..Default::default()
    });

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}
