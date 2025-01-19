use image::ImageFormat;
use mime::Mime;
use std::io::Cursor;

pub async fn compress_image(
    image_data: &[u8],
    content_type: &Mime,
    width: u32,
    height: u32,
) -> Vec<u8> {
    /*
     * I'm too lazy to fix `Mime` must be annotated with `#[derive(PartialEq)]` kind of errors
     */
    let format = if *content_type == mime::IMAGE_JPEG {
        ImageFormat::Jpeg
    } else if *content_type == mime::IMAGE_PNG {
        ImageFormat::Png
    } else {
        ImageFormat::WebP
    };

    let img = image::load_from_memory_with_format(image_data, format).expect("Invalid image data");

    /*
     * Resize and compress the image
     */
    let resized_img = img.resize(width, height, image::imageops::FilterType::Nearest);

    /*
     * Compress the image and store it in a buffer
     */
    let mut buffer = Cursor::new(Vec::new());
    resized_img
        .write_to(&mut buffer, format)
        .expect("Failed to write image");

    buffer.into_inner()
}
