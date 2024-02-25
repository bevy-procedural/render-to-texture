// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

//use crate::compress::compress_to_basis_raw;
use super::source::ImageExportSource;
use bevy::{
    ecs::query::WorldQuery,
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_asset::RenderAssets,
        render_resource::{Maintain, MapMode},
        renderer::RenderDevice,
    },
};
use futures::channel::oneshot;

#[derive(Asset, Clone, Default, Reflect, Component)]
pub struct ImageExportSettings;

impl ExtractComponent for ImageExportSettings {
    type QueryData = (&'static Self, &'static Handle<ImageExportSource>);
    type QueryFilter = ();
    type Out = (Self, Handle<ImageExportSource>);

    fn extract_component(
        (settings, source_handle): <Self::QueryData as WorldQuery>::Item<'_>,
    ) -> Option<Self::Out> {
        Some((settings.clone(), source_handle.clone_weak()))
    }
}

#[derive(Bundle, Default)]
pub struct ImageExportBundle {
    pub source: Handle<ImageExportSource>,
    pub settings: ImageExportSettings,
}

#[derive(Resource, Clone, Default, Reflect)]
pub struct ExtractableImages {
    pub raw: Vec<u8>,
    pub refresh: bool,
}

pub fn store_in_img(
    export_bundles: Query<(&Handle<ImageExportSource>, &ImageExportSettings)>,
    sources: Res<RenderAssets<ImageExportSource>>,
    render_device: Res<RenderDevice>,
    mut extractable_image: ResMut<ExtractableImages>,
    mut gpu_images: ResMut<RenderAssets<Image>>,
) {
    // TODO: don't copy the image data if it hasn't changed

    let mut copied = false;
    for (source_handle, _) in &export_bundles {
        if let Some(gpu_source) = sources.get(source_handle) {
            let mut image_bytes = {
                let slice = gpu_source.buffer.slice(..);
                {
                    let (mapping_tx, mapping_rx) = oneshot::channel();
                    render_device.map_buffer(&slice, MapMode::Read, move |res| {
                        mapping_tx.send(res).unwrap();
                    });
                    render_device.poll(Maintain::Wait);
                    futures_lite::future::block_on(mapping_rx).unwrap().unwrap();
                }
                slice.get_mapped_range().to_vec()
            };

            gpu_source.buffer.unmap();

            let bytes_per_row = gpu_source.bytes_per_row as usize;
            let padded_bytes_per_row = gpu_source.padded_bytes_per_row as usize;
            let source_size = gpu_source.source_size;
            if bytes_per_row != padded_bytes_per_row {
                let mut unpadded_bytes =
                    Vec::<u8>::with_capacity(source_size.height as usize * bytes_per_row);
                for padded_row in image_bytes.chunks(padded_bytes_per_row) {
                    unpadded_bytes.extend_from_slice(&padded_row[..bytes_per_row]);
                }
                image_bytes = unpadded_bytes;
            }

            if extractable_image.refresh && extractable_image.raw != image_bytes {
                assert!(copied == false, "Image data was copied twice");
                extractable_image.raw = image_bytes.clone();
                println!("Image data copied");
                copied = true;

                let gpu_image = gpu_images.get_mut(&gpu_source.source_handle).unwrap();
                let width = gpu_image.size.x as u32;
                let height = gpu_image.size.y as u32;
                let mut writer =
                    std::io::BufWriter::new(std::fs::File::create("test.png").unwrap());
                image::write_buffer_with_format(
                    &mut writer,
                    &image_bytes,
                    height,
                    width,
                    image::ColorType::Rgba8,
                    image::ImageFormat::Png,
                )
                .unwrap();
            }
        }
    }
}
