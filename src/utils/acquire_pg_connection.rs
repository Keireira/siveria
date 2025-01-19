use crate::service::data_providers::WebDataPool;
use actix_web::{web, Error};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use serde_json::json;

pub fn system_error() -> actix_web::Error {
    actix_web::error::InternalError::new(
        json!({"errors": { "system": "An error has been occurred. Please try again later" }}),
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    )
    .into()
}

pub async fn acquire_pg_connection(
    dp: &web::Data<WebDataPool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    dp.pg.get().map_err(|_| system_error())
}
