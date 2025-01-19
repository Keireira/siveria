use super::{heartbeat, postgres};

use actix_web::{web, Scope};

pub fn get_routes() -> Scope {
    let health_routes = web::scope("/health")
        .service(web::resource("/heartbeat").route(web::get().to(heartbeat)))
        .service(web::resource("/psql").route(web::get().to(postgres)));

    health_routes
}
