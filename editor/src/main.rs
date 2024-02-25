use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use bevy_panorbit_camera::*;
use rand::Rng;
use render_to_texture::*;
use std::f32::consts::PI;

#[derive(Component)]
struct TemporaryResource;

#[derive(Component)]
struct RenderedEntity(Handle<Mesh>);

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
    .add_systems(
        Update,
        (keyboard_input, wait_for_texture, bevy::window::close_on_esc),
    )
    .run();
}

fn wait_for_texture(
    mut commands: Commands,
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    removeables: Query<Entity, With<TemporaryResource>>,
) {
    if let Some(image) = render_to_texture_tasks.image("default", false) {
        for entity in removeables.iter() {
            commands.entity(entity).despawn();
        }

        let m = meshes.add(Mesh::from(Plane3d::new(Vec3::new(0.0, 1.0, 0.0))));
        let mm = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(image)),
            ..default()
        });

        for x in -15..15 {
            for z in -20..3 {
                commands.spawn((
                    PbrBundle {
                        mesh: m.clone(),
                        material: mm.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 * 2.0,
                            0.0,
                            z as f32 * 2.0,
                        )),
                        ..default()
                    },
                    TemporaryResource,
                ));
            }
        }
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rendered: Query<&RenderedEntity>,
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
) {
    if keys.just_pressed(KeyCode::Space) {
        create_random_mesh(&mut meshes, &mut rendered);
        render_to_texture_tasks
            .get_mut("default")
            .unwrap()
            .rerender();
    }
}

fn create_random_mesh(meshes: &mut ResMut<Assets<Mesh>>, rendered: &mut Query<&RenderedEntity>) {
    let mut rng = rand::thread_rng();
    let shape = RegularPolygon::new(rng.gen::<f32>() * 300.0, rng.gen::<usize>() % 10 + 3);
    for handle in rendered.iter() {
        let m = meshes.get_mut(handle.0.clone()).unwrap();
        *m = Mesh::from(shape.clone());
    }
}

fn create_random_texture(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    render_to_texture_tasks: &mut ResMut<RenderToTextureTasks>,
) {
    let mut rng = rand::thread_rng();

    render_to_texture_tasks.add(
        "default".to_string(),
        512,
        512,
        rng.gen(),
        commands,
        images,
        true,
    );

    let mesh = meshes.add(RegularPolygon::new(
        rng.gen::<f32>() * 300.0,
        rng.gen::<usize>() % 10 + 3,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh.clone()),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(-0.6, 0.7, 1.4),
            ..default()
        },
        RenderLayers::layer(1),
        RenderedEntity(mesh),
    ));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials2: ResMut<Assets<ColorMaterial>>,
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
) {
    create_random_texture(
        &mut commands,
        &mut images,
        &mut meshes,
        &mut materials2,
        &mut render_to_texture_tasks,
    );

    let mesh = meshes.add(Mesh::from(Cylinder::default()));
    commands.spawn((PbrBundle {
        mesh,
        material: materials.add(StandardMaterial { ..default() }),
        transform: Transform::from_xyz(-0.6, 0.7, 1.4),
        ..default()
    },));

    commands.insert_resource(AmbientLight::default());
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
