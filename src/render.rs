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
    camera: Option<Entity>,
    layer: u8,
}

impl RenderToTextureTask {
    pub fn create(
        width: u32,
        height: u32,
        commands: &mut Commands,
        images: &mut ResMut<Assets<Image>>,
    ) -> (Self, RenderLayers) {
        // TODO: always 1 is not sufficient
        let layer = 1;
        let (target, first_pass_layer, camera_id) =
            create_render_texture(width, height, commands, images, layer);

        (
            Self {
                width,
                height,
                layer,
                target,
                camera: Some(camera_id),
                stage: RenderToTextureTaskStage::Initial,
                ..Default::default()
            },
            first_pass_layer,
        )
    }
}

#[derive(Default, Resource, Reflect, Clone)]
pub struct RenderToTextureTasks {
    pub tasks: Vec<RenderToTextureTask>,
}

impl RenderToTextureTasks {
    pub fn add(
        &mut self,
        width: u32,
        height: u32,
        commands: &mut Commands,
        images: &mut ResMut<Assets<Image>>,
    ) -> RenderLayers {
        let (task, layer) = RenderToTextureTask::create(width, height, commands, images);
        self.tasks.push(task);
        return layer;
    }
}

pub fn update_render_to_texture(
    mut tasks: ResMut<RenderToTextureTasks>,
    mut assets: ResMut<Assets<Mesh>>,
    mut cameras: Query<&mut Camera>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut image_exports: ResMut<Assets<ImageExportSource>>,
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
            if let Ok(mut cam) = cameras.get_mut(task.camera.unwrap()) {
                // TODO: this isn't always transferred in time...  How to make sure the camera is turned on in time?
                println!("Activating camera");
                cam.is_active = true;
                task.stage = RenderToTextureTaskStage::Ready;

                commands.spawn(ImageExportBundle {
                    source: image_exports.add(ImageExportSource(task.target.clone())),
                    settings: crate::gpu2cpu::ImageExportSettings::default(),
                });
            }
        }
    }
}

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
        ))
        .id();

    return (image_handle, first_pass_layer, camera_id);
}
