pub mod routes;
pub mod schema;
pub mod service;
pub mod types;
pub mod utils;

extern crate diesel;
use actix_web::{self, web, App, HttpServer};
use routes::examples;
use utils::check_port_in_use;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
     * Telemetry initialization
     */
    let tel_subscriber = service::telemetry::get_subscriber();
    service::telemetry::init_subscriber(tel_subscriber);

    /*
     * Load environment variables
     */
    tracing::event!(target: "[BOOTSTRAP]", tracing::Level::INFO, "Reading ENV configuration");
    let env_config = service::env::EnvConfig::new().await;
    tracing::event!(target: "[BOOTSTRAP]", tracing::Level::INFO, "ENV configuration has been read successfully");

    /*
     * Check if port is in use
     */
    let (is_in_use, process_info) = check_port_in_use(&env_config.hostname, &env_config.port);
    if is_in_use {
        tracing::event!(target: "[BOOTSTRAP]", tracing::Level::ERROR, "Port {} is already in use!", env_config.port);
        tracing::event!(target: "[BOOTSTRAP]", tracing::Level::ERROR, "Process information:\n{}", process_info);
        std::process::exit(1);
    }

    /*
     * Run migrations for the TOWER DB
     */
    service::in_deploy_migrations::run_migrations(
        env_config.db_url.clone(),
        env_config.with_migration,
    );

    /*
     * Create data providers, so one can use them in the routes
     */
    tracing::event!(target: "[BOOTSTRAP]", tracing::Level::INFO, "Creating Web Data pool");
    let data_providers = async {
        let pool: service::data_providers::WebDataPool =
            service::data_providers::WebDataPool::new(&env_config)
                .await
                .into();

        pool
    }
    .await;
    let data_providers = web::Data::new(data_providers);
    tracing::event!(target: "[BOOTSTRAP]", tracing::Level::INFO, "All pools have been created");

    /*
     * Clone env_config, so it can be used in the closure
     */
    let bind_env_config = env_config.clone();

    HttpServer::new(move || {
        let middlewares = service::middlewares::Middlewares::new(&env_config);

        App::new()
            .wrap(middlewares.governor)
            .wrap(middlewares.cors)
            .wrap(middlewares.compress)
            .wrap(middlewares.logger)
            .wrap(middlewares.session)
            .app_data(data_providers.clone())
            .service(examples::_routes::get_routes())
    })
    .bind((bind_env_config.hostname.as_str(), bind_env_config.port))?
    .workers(1)
    .run()
    .await
}
