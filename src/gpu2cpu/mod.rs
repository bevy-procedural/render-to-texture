// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

use crate::gpu2cpu::fetch::ExtractableImage;
use bevy::{
    prelude::*,
    render::{
        camera::CameraUpdateSystem, extract_component::ExtractComponentPlugin,
        extract_resource::ExtractResourcePlugin, graph::CameraDriverLabel,
        render_asset::RenderAssetPlugin, render_graph::RenderGraph, Render, RenderApp, RenderSet,
    },
};
pub use fetch::ImageExportBundle;
use fetch::{store_in_img, ImageExportSettings};
use node::{ImageExportNode, ImageExportRenderLabel};
pub use source::ImageExportSource;
mod fetch;
mod node;
mod source;


/// Plugin enabling the generation of image sequences.
#[derive(Default)]
pub struct ImageExportPlugin {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ImageExportSystems {
    SetupImageExport,
    SetupImageExportFlush,
}

fn setup(mut commands: Commands) {
    commands.insert_resource(ExtractableImage(vec![]));
}

fn check_vec_len(extracted: Res<ExtractableImage>) {
    println!("Extracted image data: {:?}", extracted.0);
}

impl Plugin for ImageExportPlugin {
    fn build(&self, app: &mut App) {
        use ImageExportSystems::*;

        app.configure_sets(
            PostUpdate,
            (SetupImageExport, SetupImageExportFlush)
                .chain()
                .before(CameraUpdateSystem),
        )
        .register_type::<ImageExportSource>()
        .init_asset::<ImageExportSource>()
        .register_asset_reflect::<ImageExportSource>()
        //.insert_resource(ExtractedImages::default())
        .register_type::<ExtractableImage>()
        .add_plugins(ExtractResourcePlugin::<ExtractableImage>::default())
        .add_systems(Startup, setup)
        .add_plugins((
            RenderAssetPlugin::<ImageExportSource>::default(),
            ExtractComponentPlugin::<ImageExportSettings>::default(),
        ))
        .add_systems(PostUpdate, apply_deferred.in_set(SetupImageExportFlush))
        .add_systems(PreUpdate, check_vec_len);

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(
            Render,
            store_in_img
                .after(RenderSet::Render)
                .before(RenderSet::Cleanup),
        );

        let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
        graph.add_node(ImageExportRenderLabel, ImageExportNode);
        graph.add_node_edge(CameraDriverLabel, ImageExportRenderLabel);
    }
}
