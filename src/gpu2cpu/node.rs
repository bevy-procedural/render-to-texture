// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

use super::source::ImageExportSource;
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_graph::{Node, NodeRunError, RenderGraphContext, RenderLabel},
        render_resource::{ImageCopyBuffer, ImageDataLayout},
        renderer::RenderContext,
    },
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct ImageExportRenderLabel;

pub struct ImageExportNode;
impl Node for ImageExportNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        for (_, source) in world.resource::<RenderAssets<ImageExportSource>>().iter() {
            // TODO: only copy when the source has changed
            if let Some(gpu_image) = world
                .resource::<RenderAssets<Image>>()
                .get(&source.source_handle)
            {
                render_context.command_encoder().copy_texture_to_buffer(
                    gpu_image.texture.as_image_copy(),
                    ImageCopyBuffer {
                        buffer: &source.buffer,
                        layout: ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(source.padded_bytes_per_row),
                            rows_per_image: None,
                        },
                    },
                    source.source_size,
                );
            }
        }

        Ok(())
    }
}
