use super::{heartbeat, postgres, s3_upload};

// use actix_multipart::form::MultipartFormConfig;
use actix_web::{web, Scope};

pub fn get_routes() -> Scope {
    let health_routes = web::scope("/health")
        .service(web::resource("/heartbeat").route(web::get().to(heartbeat)))
        .service(web::resource("/psql").route(web::get().to(postgres)))
        // route with 5mb limit per file
        .service(
            web::resource("/s3")
                // .app_data(MultipartFormConfig::default().total_limit(5 * 1024 * 1024))
                .route(web::post().to(s3_upload)),
        );

    health_routes
}
