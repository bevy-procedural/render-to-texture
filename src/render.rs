use std::default;

use crate::gpu2cpu::{ImageExportBundle, ImageExportSource};
use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

#[derive(Component, Reflect)]
struct RendererCamera;

#[derive(Default, Reflect, Clone, PartialEq)]
pub enum RenderToTextureTaskStage {
    #[default]
    Initial,
    Ready,
    Rendering,
    Done,
}

#[derive(Default, Reflect, Clone)]
pub struct RenderToTextureTask {
    pub width: u32,
    pub height: u32,
    pub target: Handle<Image>,
    pub stage: RenderToTextureTaskStage,
    layer: u8,
}

#[derive(Default, Resource, Reflect, Clone)]
pub struct RenderToTextureTasks {
    pub tasks: Vec<RenderToTextureTask>,
}

/*
pub fn update_render_to_texture(
    mut tasks: ResMut<RenderToTextureTasks>,
    mut assets: ResMut<Assets<Mesh>>,
    mut cameras: Query<&mut Camera>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // remove finished tasks
    tasks
        .tasks
        .retain(|task| task.stage != RenderToTextureTaskStage::Done);
    assert!(
        tasks.tasks.len() <= 1,
        "Currently only one render to texture at a time is supported"
    );
    for task in tasks.tasks.iter_mut() {
        if task.stage == RenderToTextureTaskStage::Initial {
            // TODO: always 1 is not sufficient
            task.layer = 1;
            let (image_handle, first_pass_layer, camera_id) = create_render_texture(
                task.width,
                task.height,
                &mut commands,
                &mut images,
                task.layer,
            );
            task.target = image_handle;
            task.stage = RenderToTextureTaskStage::Ready;
        }
        /*if let Ok(mut cam) = cameras.get_mut(settings.camera.unwrap()) {
            // TODO: this isn't always transferred in time...  How to make sure the camera is turned on in time?
            println!("Activating camera");
            cam.is_active = true;
        }*/
    }
}*/

pub fn create_render_texture(
    width: u32,
    height: u32,
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    layer: u8,
) -> (Handle<Image>, RenderLayers, Entity) {
    let size = Extent3d {
        width,
        height,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(layer);

    let camera_id = commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d { ..default() },
                camera: Camera {
                    // render before the "main pass" camera
                    order: -1,
                    clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                    target: image_handle.clone().into(),
                    ..default()
                },
                ..default()
            },
            first_pass_layer,
            RendererCamera,
        ))
        .id();

    return (image_handle, first_pass_layer, camera_id);
}
