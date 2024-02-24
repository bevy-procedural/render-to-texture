use bevy::prelude::*;
use components::*;
use fern::{fern_mesh, FernPart};
use gpu2cpu::ImageExportSource;
pub mod fern;
mod gpu2cpu;
mod setup;

#[no_mangle]
pub fn update_render_to_texture_off(query: Query<&FernSettings>, mut cameras: Query<&mut Camera>) {
    for settings in query.iter() {
        if let Ok(mut cam) = cameras.get_mut(settings.camera.unwrap()) {
            if !cam.is_active {
                continue;
            }
            println!("Deactivating camera");
            cam.is_active = false;
        }
    }
}

#[no_mangle]
pub fn update_render_to_texture(
    query: Query<&FernSettings, Changed<FernSettings>>,
    mut assets: ResMut<Assets<Mesh>>,
    mut cameras: Query<&mut Camera>,
) {
    for settings in query.iter() {
        if let Ok(mut cam) = cameras.get_mut(settings.camera.unwrap()) {
            // TODO: this isn't always transferred in time...  How to make sure the camera is turned on in time?
            println!("Activating camera");
            cam.is_active = true;
        }

        //println!("Updating fern mesh");
        let fern = fern_mesh(settings, FernPart::Stem);
        let mesh = assets.get_mut(settings.meshes[0]).unwrap();
        fern.bevy_set(mesh);

        let fern = fern_mesh(settings, FernPart::LeafletTop);
        let mesh = assets.get_mut(settings.meshes[1]).unwrap();
        fern.bevy_set(mesh);

        let fern = fern_mesh(settings, FernPart::LeafletBottom);
        let mesh = assets.get_mut(settings.meshes[2]).unwrap();
        fern.bevy_set(mesh);
    }
}

#[no_mangle]
pub fn make_fern_material(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    images: &mut ResMut<Assets<Image>>
) -> (
    bevy::pbr::ExtendedMaterial<StandardMaterial, FernMaterial>,
    Mesh,
) {
    setup::make_fern_material_internal(commands, meshes, materials, images)
}
