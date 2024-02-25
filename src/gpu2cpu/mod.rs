// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

use bevy::{
    prelude::*,
    render::{
        camera::CameraUpdateSystem, extract_component::ExtractComponentPlugin,
        graph::CameraDriverLabel, render_asset::RenderAssetPlugin, render_graph::RenderGraph,
        MainWorld, Render, RenderApp, RenderSet,
    },
};
use fetch::store_in_img;
pub use fetch::{ExtractableImages, ImageExportBundle, ImageExportSettings};
use node::{ImageExportNode, ImageExportRenderLabel};
pub use source::ImageExportSource;
mod fetch;
mod node;
mod source;

#[derive(Default)]
pub struct ImageExportPlugin {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ImageExportSystems {
    SetupImageExport,
    SetupImageExportFlush,
}

pub fn sync_images(render_world_data: Res<ExtractableImages>, mut world: ResMut<MainWorld>) {
    if render_world_data.raw.is_empty() {
        return;
    }
    let mut main_world_data = world.get_resource_mut::<ExtractableImages>().unwrap();

    if !main_world_data.raw.is_empty() {
        return;
    }
    main_world_data.raw = render_world_data.raw.clone();

    // TODO: clear the data after it's been copied
    // render_world_data.raw.clear();
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
        .add_plugins((
            RenderAssetPlugin::<ImageExportSource>::default(),
            ExtractComponentPlugin::<ImageExportSettings>::default(),
        ))
        .add_systems(PostUpdate, apply_deferred.in_set(SetupImageExportFlush));

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<ExtractableImages>()
            .add_systems(ExtractSchedule, sync_images)
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
