//! PNG (sRGB) and EXR (linear) decode for import tests.

use std::io::Cursor;

use exr::image::pixel_vec::PixelVec;
use exr::prelude::{
    read, Compression, Encoding, Image, LineOrder, ReadChannels, ReadLayers, SpecificChannels,
    WritableImage,
};
use png::ColorType;

/// Tagged color space after decode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSpace {
    /// sRGB transfer.
    Srgb,
    /// Scene-linear.
    Linear,
}

/// Decoded RGBA f32 pixels (linear scene values).
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedTexture {
    /// Interpretation of RGB channels.
    pub color_space: ColorSpace,
    /// Row-major RGBA, linear floats.
    pub pixels: Vec<[f32; 4]>,
}

fn srgb_to_linear_u8(c: u8) -> f32 {
    let x = f32::from(c) / 255.0;
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

/// Decode a PNG that is tagged or assumed sRGB; linearizes channels.
pub fn decode_png_srgb(png_bytes: &[u8]) -> Result<DecodedTexture, String> {
    let decoder = png::Decoder::new(png_bytes);
    let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
    if info.color_type != ColorType::Rgba {
        return Err("expected RGBA PNG".into());
    }
    let mut pixels = Vec::new();
    for chunk in buf[..info.buffer_size()].chunks_exact(4) {
        pixels.push([
            srgb_to_linear_u8(chunk[0]),
            srgb_to_linear_u8(chunk[1]),
            srgb_to_linear_u8(chunk[2]),
            f32::from(chunk[3]) / 255.0,
        ]);
    }
    Ok(DecodedTexture {
        color_space: ColorSpace::Srgb,
        pixels,
    })
}

/// Encode a 1×1 RGB f32 EXR (uncompressed).
pub fn write_exr_linear_fixture(r: f32, g: f32, b: f32) -> Result<Vec<u8>, String> {
    let mut file_bytes = Vec::new();
    let image = Image::from_encoded_channels(
        (1, 1),
        Encoding {
            compression: Compression::Uncompressed,
            line_order: LineOrder::Increasing,
            ..Encoding::default()
        },
        SpecificChannels::rgb(PixelVec::new((1, 1), vec![(r, g, b)])),
    );
    image
        .write()
        .to_buffered(Cursor::new(&mut file_bytes))
        .map_err(|e| format!("{e:?}"))?;
    Ok(file_bytes)
}

/// Decode a 1×1 RGB f32 EXR produced by [`write_exr_linear_fixture`].
pub fn decode_exr_linear(exr_bytes: &[u8]) -> Result<DecodedTexture, String> {
    let img = read()
        .no_deep_data()
        .largest_resolution_level()
        .rgb_channels(
            PixelVec::<(f32, f32, f32)>::constructor,
            PixelVec::set_pixel,
        )
        .first_valid_layer()
        .all_attributes()
        .from_buffered(Cursor::new(exr_bytes))
        .map_err(|e| format!("{e:?}"))?;
    let p = img.layer_data.channel_data.pixels.pixels[0];
    Ok(DecodedTexture {
        color_space: ColorSpace::Linear,
        pixels: vec![[p.0, p.1, p.2, 1.0]],
    })
}
