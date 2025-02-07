use std::fs::File;
use std::io::prelude::Write;

use image::{self, ImageEncoder};
use svg2pdf::{self, ConversionOptions, PageOptions};

pub fn save_jpeg(
    file: &mut File,
    data: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<(), std::io::Error> {
    let mut jpeg_data = Vec::new();
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_data, 100);
    encoder
        .write_image(
            data.as_slice(),
            width,
            height,
            image::ExtendedColorType::Rgb8,
        )
        .unwrap();

    file.write_all(&jpeg_data)?;
    Ok(())
}

pub fn save_png(
    file: &mut File,
    data: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<(), std::io::Error> {
    let mut png_data = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new_with_quality(
        &mut png_data,
        image::codecs::png::CompressionType::Fast,
        image::codecs::png::FilterType::NoFilter,
    );
    encoder
        .write_image(
            data.as_slice(),
            width,
            height,
            image::ExtendedColorType::Rgba8,
        )
        .unwrap();

    file.write_all(&png_data)?;
    Ok(())
}

pub fn save_webp(
    file: &mut File,
    data: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<(), std::io::Error> {
    let mut webp_data = Vec::new();
    let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut webp_data);
    encoder
        .write_image(
            data.as_slice(),
            width,
            height,
            image::ExtendedColorType::Rgba8,
        )
        .unwrap();

    file.write_all(&webp_data)?;
    Ok(())
}

pub fn save_pdf(file: &mut File, svg: String) -> Result<(), std::io::Error> {
    let mut options = svg2pdf::usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = svg2pdf::usvg::Tree::from_str(&svg, &options).unwrap();
    let pdf = svg2pdf::to_pdf(&tree, ConversionOptions::default(), PageOptions::default()).unwrap();

    file.write_all(&pdf)?;
    Ok(())
}
