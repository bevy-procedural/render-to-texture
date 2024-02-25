// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

use crate::gpu2cpu::fetch::ExtractableImages;
use bevy::{
    prelude::*,
    render::{
        camera::CameraUpdateSystem, extract_component::ExtractComponentPlugin,
        extract_resource::ExtractResourcePlugin, graph::CameraDriverLabel,
        render_asset::RenderAssetPlugin, render_graph::RenderGraph, Extract, MainWorld, Render,
        RenderApp, RenderSet,
    },
};
use fetch::store_in_img;
pub use fetch::{ImageExportBundle, ImageExportSettings};
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
    commands.insert_resource(ExtractableImages::default());
}

fn check_vec_len(extracted: Res<ExtractableImages>) {
    println!("Extracted image data: {:?}", extracted.raw.len());
}

pub fn sync_images2(render_world: Res<ExtractableImages>, mut world: ResMut<MainWorld>) {
    let mut images = world.get_resource_mut::<ExtractableImages>().unwrap();
    images.raw = render_world.raw.clone();
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
        .register_type::<ExtractableImages>()
        .insert_resource(ExtractableImages::default())
        //.add_plugins(ExtractResourcePlugin::<ExtractableImages>::default())
        .add_systems(Startup, setup)
        .add_plugins((
            RenderAssetPlugin::<ImageExportSource>::default(),
            ExtractComponentPlugin::<ImageExportSettings>::default(),
        ))
        .add_systems(PostUpdate, apply_deferred.in_set(SetupImageExportFlush))
        .add_systems(PreUpdate, check_vec_len);

        let render_app = app.sub_app_mut(RenderApp);

        // app.insert_sub_app(RenderApp, SubApp::new(sub_app, sync_chunks));

        render_app
            .init_resource::<ExtractableImages>()
            .add_systems(ExtractSchedule, sync_images2)
            .add_systems(
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
