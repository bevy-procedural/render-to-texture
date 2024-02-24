/// To make changes to the systems not break the type ids of components, making a components sub-crate is recommended. This way, they are a separate compilation unit. Otherwise component queries might suddenly be empty after code changes.
use bevy::{
    pbr::{MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError},
    },
};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FernMaterial {
    #[uniform(100)]
    pub time: f32,
}

#[derive(Component)]
pub struct Fern;

impl MaterialExtension for FernMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/fern.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/fern_prepass.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // disable backface culling
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct FernSettings {
    #[inspector(min = 0.001, max = 0.3, speed = 0.001)]
    pub stem_w: f32,
    #[inspector(min = 0.001, max = 0.3, speed = 0.001)]
    pub stem_w2: f32,

    #[inspector(min = 2, max = 100)]
    pub leaflets1: u32,
    #[inspector(min = 3, max = 100)]
    pub leaflets2: u32,
    #[inspector(min = 0.0, max = 10.0, speed = 0.001)]
    pub leaflet_spacing: f32,
    #[inspector(min = 0.0, max = 10.0, speed = 0.001)]
    pub leafshape_exp: f32,
    #[inspector(min = 0.0, max = 10.0, speed = 0.001)]
    pub curvature: f32,

    #[inspector(min = -10.0, max = 10.0, speed = 0.001)]
    pub slant: f32,
    #[inspector(min = 0.0, max = 10.0, speed = 0.001)]
    pub thinning: f32,
    #[inspector(min = 0.0, max = 10.0, speed = 0.001)]
    pub stomp: f32,
    #[inspector(min = 0.0, max = 10.0, speed = 0.00001)]
    pub l0: f32,

    #[inspector(min = 8, max = 4096)]
    pub width: u32,
    #[inspector(min = 8, max = 4096)]
    pub height: u32,

    pub meshes: Vec<AssetId<Mesh>>,
    pub camera: Option<Entity>,
    pub render_target: Option<Handle<Image>>,
    // To enable automatic reloading
    pub version: u32,
}

impl Default for FernSettings {
    fn default() -> Self {
        FernSettings {
            stem_w: 0.08,
            stem_w2: 0.008,
            leaflets1: 35,
            leaflets2: 10,
            leaflet_spacing: 1.6,
            leafshape_exp: 1.5,
            curvature: 0.014,

            slant: 2.0,
            thinning: 0.5,
            stomp: 1.4,
            l0: 0.0521,

            width: 512,
            height: 512,
            meshes: vec![],
            camera: None,
            render_target: None,
            version: 0,
        }
    }
}

#[derive(Component)]
pub struct MainCamera;
