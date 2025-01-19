use crate::service::data_providers::WebDataPool;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, Error, HttpResponse};
use serde_derive::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::utils::files::{get_extension_from_mime, get_file_bytes};
use crate::utils::images::compress_image;

#[derive(MultipartForm)]
pub struct UserForm {
    pub image: Option<TempFile>,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub image_url: String,
}

pub async fn s3_upload(
    user: MultipartForm<UserForm>,
    dp: web::Data<WebDataPool>,
) -> Result<HttpResponse, Error> {
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB

    /*
     * Ensure the image file exists
     */
    let image_file = user.image.as_ref().ok_or_else(|| {
        actix_web::error::ErrorBadRequest(json!({
            "message": "No image file"
        }))
    })?;

    /*
     * Check if the content type is allowed (images only)
     */
    let content_type = image_file.content_type.as_ref().ok_or_else(|| {
        actix_web::error::ErrorBadRequest(json!({
            "message": "Content type is missing"
        }))
    })?;

    if content_type.type_() != mime::IMAGE {
        return Err(actix_web::error::ErrorBadRequest(json!({
            "message": "Wrong file type"
        })));
    }

    /*
     * Read the file bytes
     */
    let bytes = get_file_bytes(&image_file).await.map_err(|_| {
        actix_web::error::ErrorBadRequest(json!({
            "error": "Error reading file bytes",
        }))
    })?;

    /*
     * Check the file size (5 Mb only)
     */
    if bytes.len() > MAX_FILE_SIZE {
        return Err(actix_web::error::ErrorPayloadTooLarge(json!({
            "error": "File too large",
        })));
    }

    /*
     * Determine the file extension
     */
    let file_extension = get_extension_from_mime(content_type.as_ref()).ok_or_else(|| {
        actix_web::error::ErrorBadRequest(json!({
            "message": "Could not determine file extension."
        }))
    })?;

    /*
     * Generate the file name to upload
     */
    let uuid_name = Uuid::new_v4().to_string();
    let file_name_to_upload = format!("data/{}.{}", uuid_name, file_extension);

    /*
     * Compress the image
     */
    let compressed_bytes = compress_image(&bytes, file_extension, 640, None).await;

    /*
     * Upload the file to the bucket
     */
    let _result = dp
        .s3
        .put_object(&file_name_to_upload, &compressed_bytes)
        .await
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError(json!({
                "message": "Error uploading file."
            }))
        })?;

    /*
     * Construct the URL to the uploaded file
     */
    let completed_url = format!("{}/{}", dp.s3.url(), file_name_to_upload);

    Ok(actix_web::HttpResponse::Ok().json(UploadResponse {
        image_url: completed_url,
    }))
}
