use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::{CascadeShadowConfigBuilder, ExtendedMaterial},
    prelude::*,
    render::view::NoFrustumCulling,
    window::WindowResolution,
};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_panorbit_camera::*;
use std::{env, f32::consts::PI};

#[cfg(not(feature = "reload"))]
pub use components::*;
#[cfg(not(feature = "reload"))]
use render_to_texture::*;
#[cfg(feature = "reload")]
use render_to_texture_hot::*;
#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(
    dylib = "render_to_texture",
    file_watch_debounce = 200,
    lib_dir = "target/debug"
)]
mod render_to_texture_hot {
    use bevy::prelude::*;
    pub use components::*;
    hot_functions_from_file!("src/lib.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}
}

#[cfg(feature = "reload")]
fn reload_after_change(mut query: Query<&mut FernSettings>) {
    if render_to_texture_hot::was_updated() {
        println!("Reloading systems");
        for mut settings in query.iter_mut() {
            settings.version = settings.version + 1;
        }
    }
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full"

    #[cfg(feature = "reload")]
    println!("Hello from the main module! This is a hot reloadable module.");

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
    /*.insert_resource(WireframeConfig {
        global: true,
        default_color: Color::WHITE,
    })*/
    .add_plugins(MaterialPlugin::<
        ExtendedMaterial<StandardMaterial, FernMaterial>,
    >::default())
    .register_type::<FernSettings>()
    .add_systems(Startup, setup_scene)
    .add_plugins((
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin::default(),
        FilterQueryInspectorPlugin::<With<FernSettings>>::default(),
        PanOrbitCameraPlugin,
    ));

    #[cfg(feature = "reload")]
    app.add_systems(PreUpdate, reload_after_change);

    app.add_systems(
        Update,
        (
            update_render_to_texture_off,
            update_render_to_texture.after(update_render_to_texture_off),
            bevy::window::close_on_esc,
        ),
    )
    .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FernMaterial>>>,
    mut materials3: ResMut<Assets<ColorMaterial>>,
    mut materials2: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>
) {
    // TODO: use instancing https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/shader/shader_instancing.rs#L104

    let (material, mesh) = make_fern_material(
        &mut commands,
        &mut meshes,
        &mut materials3,
        &mut images
    );
    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    for i in 0..30 {
        let s = (i as f32 * 100.0).sin() + 2.0;

        commands.spawn((
            MaterialMeshBundle {
                mesh: mesh_handle.clone(),
                transform: Transform::from_xyz(
                    ((1012.0 * i as f32).sin() * 100000.0) % 10.0,
                    s / 2.0,
                    ((432.0 * i as f32).sin() * 100000.0) % 10.0,
                )
                .with_scale(Vec3::splat(s)),
                material: material_handle.clone(),
                ..default()
            },
            // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
            // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
            // instanced cubes will be culled.
            // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
            // instancing, and that is not taken into account with the built-in frustum culling.
            // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
            // component to avoid incorrect culling.
            NoFrustumCulling,
        ));
    }

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::new(0.0, 1.0, 0.0)))),
        material: materials2.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.4),
            ..default()
        }),
        transform: Transform::from_scale(Vec3::splat(100.0)),
        ..default()
    },));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::default())),
        material: materials2.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        }),
        transform: Transform::from_xyz(-0.6, 0.7, 1.4),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });

    /*commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 3.0, 0.0),
        ..default()
    });*/

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
        // High-Quality Shadows!
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.001,
            maximum_distance: 30.0,
            first_cascade_far_bound: 10.0,
            overlap_proportion: 0.2,

            ..default()
        }
        .into(),
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
