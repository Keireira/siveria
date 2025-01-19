use crate::types::users::UserForm;
use actix_web::body::BoxBody;
use actix_web::{web, HttpResponse};
use mime::Mime;
use serde_json::json;
use uuid::Uuid;

use crate::service::data_providers::WebDataPool;
use crate::service::env::EnvConfig;
use crate::utils::uploads;

fn is_allowed_file(content_type: &Mime) -> bool {
    let allowed_mime_types = vec![mime::IMAGE];

    allowed_mime_types.contains(&content_type.type_())
}

// @TODO: We use webp only, so be it
// @TODO: I want to be able to delete avatar by sending empty value for avatar key
pub async fn update_avatar(
    user: &actix_multipart::form::MultipartForm<UserForm>,
    dp: &web::Data<WebDataPool>,
    envs: &web::Data<EnvConfig>,
) -> Result<String, HttpResponse<BoxBody>> {
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 5MB

    /*
     * Ensure the avatar file exists
     */
    let temp_file = user.avatar.as_ref().ok_or_else(|| {
        HttpResponse::BadRequest().json(json!({
            "message": "No avatar file uploaded."
        }))
    })?;

    /*
     * Get the content type of the file
     */
    let content_type = temp_file.content_type.as_ref().ok_or_else(|| {
        HttpResponse::BadRequest().json(json!({
            "message": "Content type is missing."
        }))
    })?;

    /*
     * Check if the content type is allowed
     */
    if !is_allowed_file(&content_type) {
        return Err(HttpResponse::BadRequest().json(json!({
            "message": "File type not allowed."
        })));
    }

    /*
     * Read the file bytes
     */
    let bytes = uploads::get_file_bytes(&temp_file).await.map_err(|_| {
        HttpResponse::InternalServerError().json(json!({
            "message": "Error reading file bytes."
        }))
    })?;

    /*
     * Check the file size
     */
    if bytes.len() > MAX_FILE_SIZE {
        return Err(HttpResponse::PayloadTooLarge().json(json!({
            "message": "File too large."
        })));
    }

    /*
     * Determine the file extension
     */
    let file_extension =
        uploads::get_extension_from_mime(content_type.as_ref()).ok_or_else(|| {
            HttpResponse::BadRequest().json(json!({
                "message": "Could not determine file extension."
            }))
        })?;

    /*
     * Generate the file name to upload
     */
    let file_name = Uuid::new_v4().to_string();
    let file_name_to_upload = format!("media/avatars/{}.{}", file_name, file_extension);

    /*
     * Compress the image
     */
    let compressed_bytes = uploads::compress_image(&bytes, content_type, 320, 320).await;

    /*
     * Upload the file to the bucket
     */
    let _result = dp
        .s3
        .put_object(&file_name_to_upload, &compressed_bytes)
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().json(json!({
                "message": "Error uploading file."
            }))
        })?;

    /*
     * Construct the URL to the uploaded file
     */
    let completed_url = format!(
        "{}/{}/{}",
        envs.s3_endpoint_url, envs.s3_bucket_name, file_name_to_upload
    );

    Ok(completed_url)
}
