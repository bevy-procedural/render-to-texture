use bevy::prelude::*;

pub fn compress_to_basis_raw(data: &Vec<u8>, size: UVec2, is_srgb: bool) -> Vec<u8> {
    // from bevy::render::texture::CompressedImageSaver:

    // PERF: this should live inside the future, but CompressorParams and Compressor are not Send / can't be owned by the BoxedFuture (which _is_ Send)
    let mut compressor_params = basis_universal::CompressorParams::new();
    compressor_params.set_basis_format(basis_universal::BasisTextureFormat::UASTC4x4);
    compressor_params.set_generate_mipmaps(true);
    let color_space = if is_srgb {
        basis_universal::ColorSpace::Srgb
    } else {
        basis_universal::ColorSpace::Linear
    };
    compressor_params.set_color_space(color_space);
    compressor_params.set_uastc_quality_level(basis_universal::UASTC_QUALITY_DEFAULT);
    compressor_params.set_etc1s_quality_level(basis_universal::ETC1S_QUALITY_DEFAULT);

    let mut source_image = compressor_params.source_image_mut(0);
    source_image.init(data, size.x, size.y, 4);

    let mut compressor = basis_universal::Compressor::new(4);
    // SAFETY: the CompressorParams are "valid" to the best of our knowledge. The basis-universal
    // library bindings note that invalid params might produce undefined behavior.
    unsafe {
        compressor.init(&compressor_params);
        compressor.process().unwrap();
    }
    let compressed_basis_data = compressor.basis_file().to_vec();

    //println!("Original data size: {}", image.data.len());
    //println!("Compressed data size: {}", compressed_basis_data.len());

    return compressed_basis_data;
}

pub fn compress_to_basis(image: &Image) -> Vec<u8> {
    compress_to_basis_raw(
        &image.data,
        image.size(),
        image.texture_descriptor.format.is_srgb(),
    )
}