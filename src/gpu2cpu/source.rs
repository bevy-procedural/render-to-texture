// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssetUsages, RenderAssets},
        render_resource::{Buffer, BufferDescriptor, BufferUsages, Extent3d},
        renderer::RenderDevice,
    },
};

#[derive(Asset, Clone, Default, Reflect)]
pub struct ImageExportSource {
    pub image: Handle<Image>,
}

impl From<Handle<Image>> for ImageExportSource {
    fn from(value: Handle<Image>) -> Self {
        Self { image: value }
    }
}

pub struct GpuImageExportSource {
    pub buffer: Buffer,
    pub source_handle: Handle<Image>,
    pub source_size: Extent3d,
    pub bytes_per_row: u32,
    pub padded_bytes_per_row: u32,
}

impl RenderAsset for ImageExportSource {
    type PreparedAsset = GpuImageExportSource;
    type Param = (SRes<RenderDevice>, SRes<RenderAssets<Image>>);

    fn asset_usage(&self) -> bevy::render::render_asset::RenderAssetUsages {
        RenderAssetUsages::default()
    }

    fn prepare_asset(
        self,
        (device, images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self>> {
        let gpu_image = images.get(&self.image).unwrap();

        let size = gpu_image.texture.size();
        let format = &gpu_image.texture_format;
        let bytes_per_row =
            (size.width / format.block_dimensions().0) * format.block_copy_size(None).unwrap();
        let padded_bytes_per_row =
            RenderDevice::align_copy_bytes_per_row(bytes_per_row as usize) as u32;

        let source_size = gpu_image.texture.size();

        Ok(GpuImageExportSource {
            buffer: device.create_buffer(&BufferDescriptor {
                label: Some("Image Export Buffer"),
                size: (source_size.height * padded_bytes_per_row) as u64,
                usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            source_handle: self.image,
            source_size,
            bytes_per_row,
            padded_bytes_per_row,
        })
    }
}
