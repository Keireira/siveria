#[derive(actix_multipart::form::MultipartForm)]
pub struct UserForm {
    pub name: Option<actix_multipart::form::text::Text<String>>,
    pub avatar: Option<actix_multipart::form::tempfile::TempFile>,
}
