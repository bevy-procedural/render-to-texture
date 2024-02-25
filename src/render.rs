use crate::{
    compress::compress_to_basis_raw,
    gpu2cpu::{ExtractableImages, ImageExportBundle, ImageExportSource},
};
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{CompressedImageFormats, ImageSampler, ImageType},
        view::RenderLayers,
    },
    utils::HashMap,
};

#[derive(Default, Reflect, Clone, PartialEq)]
pub enum RenderToTextureTaskStage {
    #[default]
    Initialized,
    ReadyForRendering,
    RenderedResultCopiedBack,
    ReadyForReading,
    ResultReceived,
}

#[derive(Default, Reflect, Clone)]
pub struct RenderToTextureTask {
    width: u32,
    height: u32,
    should_compress: bool,
    target: Handle<Image>,
    pub stage: RenderToTextureTaskStage,
    camera: Option<Entity>,
    layer: u8,
    is_srgb: bool,
    bundle: Option<Entity>,
    data: Vec<u8>,
}

impl RenderToTextureTask {
    pub fn new(
        width: u32,
        height: u32,
        should_compress: bool,
        commands: &mut Commands,
        images: &mut ResMut<Assets<Image>>,
    ) -> Self {
        // TODO: always 1 is not sufficient
        let layer = 1;
        let (target, camera_id) =
            create_render_texture(width, height, commands, images, layer, false);

        return Self {
            width,
            height,
            layer,
            target,
            is_srgb: true, // TODO
            should_compress,
            camera: Some(camera_id),
            stage: RenderToTextureTaskStage::Initialized,
            ..Default::default()
        };
    }

    pub fn get_layer(&self) -> RenderLayers {
        RenderLayers::layer(self.layer)
    }

    pub fn size(&self) -> UVec2 {
        UVec2::new(self.width, self.height)
    }

    pub fn ready(&self) -> bool {
        self.stage == RenderToTextureTaskStage::ReadyForReading
    }
}

#[derive(Default, Resource, Clone)]
pub struct RenderToTextureTasks {
    tasks: HashMap<String, RenderToTextureTask>,
    supported_compressed_formats: CompressedImageFormats,
}

#[derive(Default, Component, Clone, Reflect)]
pub struct TaskResource(pub String);

impl RenderToTextureTasks {
    /// should_compress: whether to use universal basis compression. This will also generate mipmaps.
    pub fn add(
        &mut self,
        name: String,
        width: u32,
        height: u32,
        should_compress: bool,
        commands: &mut Commands,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let task = RenderToTextureTask::new(width, height, should_compress, commands, images);
        assert!(
            self.tasks.get(&name).is_none(),
            "Task with name {} already exists",
            name
        );
        self.tasks.insert(name, task);
    }

    pub fn get(&self, name: &str) -> Option<&RenderToTextureTask> {
        self.tasks.get(name)
    }

    pub fn read(&mut self, name: &str) -> Option<Vec<u8>> {
        if let Some(task) = self.tasks.get_mut(name) {
            if task.stage != RenderToTextureTaskStage::ReadyForReading {
                return None;
            }
            task.stage = RenderToTextureTaskStage::ResultReceived;
            return Some(task.data.clone());
        }
        return None;
    }

    pub fn image(&mut self, name: &str) -> Option<Image> {
        // TODO: Delete the image when not in use anymore

        if let Some(task) = self.tasks.get_mut(name) {
            if task.stage != RenderToTextureTaskStage::ReadyForReading {
                return None;
            }
            task.stage = RenderToTextureTaskStage::ResultReceived;
            if task.should_compress {
                return Some(
                    Image::from_buffer(
                        &task.data,
                        ImageType::Format(bevy::render::texture::ImageFormat::Basis),
                        self.supported_compressed_formats,
                        true,
                        ImageSampler::linear(), // TODO: mipmap trilinear?
                        RenderAssetUsages::default(),
                    )
                    .unwrap(),
                );
            } else {
                return Some(Image::new_fill(
                    Extent3d {
                        width: task.width,
                        height: task.height,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &task.data,
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::default(),
                ));
            }
        }
        return None;
    }
}

pub fn update_render_to_texture(
    mut tasks: ResMut<RenderToTextureTasks>,
    mut cameras: Query<&mut Camera>,
    mut commands: Commands,
    mut image_exports: ResMut<Assets<ImageExportSource>>,
    extractable_images: Res<ExtractableImages>,
) {
    // remove finished tasks
    tasks
        .tasks
        .retain(|_, task| task.stage != RenderToTextureTaskStage::ResultReceived);

    if extractable_images.raw.len() > 0 {
        for (_, task) in tasks.tasks.iter_mut() {
            if task.stage == RenderToTextureTaskStage::ReadyForRendering {
                task.data = extractable_images.raw.clone(); // TODO: avoid cloning

                let mut writer =
                    std::io::BufWriter::new(std::fs::File::create("test.png").unwrap());
                image::write_buffer_with_format(
                    &mut writer,
                    &task.data,
                    task.width,
                    task.height,
                    image::ColorType::Rgba8,
                    image::ImageFormat::Png,
                )
                .unwrap();

                // TODO: extractable_images.raw.clear();
                task.stage = RenderToTextureTaskStage::RenderedResultCopiedBack;
            }
        }
    }

    let mut started_rendering = false;
    for (_, task) in tasks.tasks.iter_mut() {
        match task.stage {
            RenderToTextureTaskStage::ReadyForRendering => {}
            RenderToTextureTaskStage::RenderedResultCopiedBack => {
                commands.entity(task.camera.unwrap()).despawn_recursive();
                commands.entity(task.bundle.unwrap()).despawn_recursive();
                // commands.remove(task.target);
                if task.should_compress {
                    // TODO: do this in a separate thread / TaskPool
                    let prev_len = task.data.len();
                    task.data = compress_to_basis_raw(&task.data, task.size(), task.is_srgb);
                    println!(
                        "Compressing... {} -> {} Kb",
                        prev_len / 1024,
                        task.data.len() / 1024
                    );
                    task.stage = RenderToTextureTaskStage::ReadyForReading;
                } else {
                    task.stage = RenderToTextureTaskStage::ReadyForReading;
                }
            }
            RenderToTextureTaskStage::Initialized => {
                if let Ok(mut cam) = cameras.get_mut(task.camera.unwrap()) {
                    assert!(
                        !started_rendering,
                        "Currently only one render to texture at a time is supported"
                    );

                    cam.is_active = true;
                    task.stage = RenderToTextureTaskStage::ReadyForRendering;

                    task.bundle = Some(commands.spawn(ImageExportBundle {
                        source: image_exports.add(ImageExportSource {
                            image: task.target.clone(),
                        }),
                        settings: crate::gpu2cpu::ImageExportSettings::default(),
                    }).id());
                    started_rendering = true;
                }
            }
            _ => {}
        };
    }
}

pub fn create_render_texture(
    width: u32,
    height: u32,
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    layer: u8,
    direct_render: bool,
) -> (Handle<Image>, Entity) {
    let size = Extent3d {
        width,
        height,
        ..default()
    };

    let mut usage = TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC;
    if direct_render {
        usage |= TextureUsages::TEXTURE_BINDING;
    }

    // TODO: Delete the image when not in use anymore

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: if direct_render {
                TextureFormat::Bgra8UnormSrgb
            } else {
                TextureFormat::Rgba8UnormSrgb
            },
            mip_level_count: 1,
            sample_count: 1,
            usage,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

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
            RenderLayers::layer(layer),
        ))
        .id();

    return (image_handle, camera_id);
}
