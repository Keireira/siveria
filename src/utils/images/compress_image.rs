use image::{imageops::FilterType, load_from_memory_with_format, ImageFormat};
use std::io::Cursor;

pub async fn compress_image(
    image_data: &[u8],
    file_ext: &str,
    width: impl Into<Option<u32>>,
    height: impl Into<Option<u32>>,
) -> Vec<u8> {
    let format = ImageFormat::from_extension(file_ext).expect("Invalid image format");
    let img = load_from_memory_with_format(image_data, format).expect("Invalid image data");

    let width = width.into().unwrap_or(img.width());
    let height = height.into().unwrap_or(img.height());

    let compressed_image = img.resize(width, height, FilterType::Nearest);

    let mut buffer = Cursor::new(Vec::new());
    compressed_image
        .write_to(&mut buffer, format)
        .expect("Failed to write image");

    buffer.into_inner()
}
