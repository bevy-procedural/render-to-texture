use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use bevy_panorbit_camera::*;
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
    .add_plugins((PanOrbitCameraPlugin, RenderToTexturePlugin))
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
        let m = meshes.add(Mesh::from(Plane3d::new(Vec3::new(0.0, 1.0, 0.0))));
        let mm = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(image)),
            ..default()
        });

        for x in -15..15 {
            for z in -20..3 {
                commands.spawn((PbrBundle {
                    mesh: m.clone(),
                    material: mm.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * 2.0,
                        0.0,
                        z as f32 * 2.0,
                    )),
                    ..default()
                },));
            }
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials2: ResMut<Assets<ColorMaterial>>,
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
            material: materials2.add(Color::RED),
            transform: Transform::from_xyz(-0.6, 0.7, 1.4),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    /*let (image_handle, _) = create_render_texture(512, 512, &mut commands, &mut images, 1);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegularPolygon::new(100.0, 6))),
            material: materials2.add(Color::RED),
            transform: Transform::from_xyz(-0.6, 0.7, 1.4),
            ..default()
        },
        RenderLayers::layer(1),
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

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
