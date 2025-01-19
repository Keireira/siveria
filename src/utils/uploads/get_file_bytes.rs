use actix_multipart::form::tempfile::TempFile;
use std::io::Read;

pub async fn get_file_bytes(file: &TempFile) -> std::io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut f = &file.file;

    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}
