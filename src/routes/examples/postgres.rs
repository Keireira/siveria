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
    /*
     * Acquire a connection to the database.
     */
    let mut connection = match acquire_pg_connection(&dp).await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::event!(target: "[HEALTH/POSTGRES]", tracing::Level::ERROR, "Failed to acquire connection: {}", e);

            return HttpResponse::InternalServerError().finish();
        }
    };

    /*
     * Query the database for all entries in the test_table.
     */
    let result = web::block(move || {
        test_table::table
            .load::<TestEntry>(&mut connection)
            .map_err(|e| e.to_string())
    })
    .await;

    /*
     * Handle the result of the query.
     */
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
