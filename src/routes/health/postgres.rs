use crate::schema::test_table;
use crate::service::data_providers::WebDataPool;
use crate::utils::acquire_pg_connection;
use serde_derive::Serialize;

use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestEntry {
    pub id: Uuid,
    pub name: String,
}

pub async fn postgres(dp: web::Data<WebDataPool>) -> actix_web::HttpResponse {
    let mut conn = acquire_pg_connection(&dp).await.unwrap();

    let result = web::block(move || {
        test_table::table
            .load::<TestEntry>(&mut conn)
            .map_err(|e| e.to_string())
    })
    .await;

    match result {
        Ok(Ok(entries)) => HttpResponse::Ok().json(entries),
        Ok(Err(e)) => {
            tracing::event!(target: "[HEALTH/POSTGRES]", tracing::Level::ERROR, "Database error: {}", e);

            HttpResponse::InternalServerError().finish()
        }
        Err(e) => {
            tracing::event!(target: "[HEALTH/POSTGRES]", tracing::Level::ERROR, "Blocking error: {}", e);

            HttpResponse::InternalServerError().finish()
        }
    }
}
