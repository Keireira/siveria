use serde_derive::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub is_healthy: bool,
    pub text: String,
}

pub async fn heartbeat() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(HealthResponse {
        is_healthy: true,
        text: "Application is safe and sound".to_string(),
    })
}
