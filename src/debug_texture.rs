use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::ImageSampler}};

/// Creates a colorful test pattern
pub fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 16;

    let color_black: [u8; 4] = [0, 0, 0, 255];
    let color_purle: [u8; 4] = [165, 6, 155, 255];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let offset = (x + TEXTURE_SIZE * y) * 4;

            if (x <= 7 && y <= 7) || (x > 7 && y > 7) {
                texture_data[offset..(offset + 4)].copy_from_slice(&color_black);
            } else {
                texture_data[offset..(offset + 4)].copy_from_slice(&color_purle);
            }
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    );

    image.sampler_descriptor = ImageSampler::nearest();

    image
}