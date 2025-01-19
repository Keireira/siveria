use actix_cors::Cors;
use actix_governor::governor::clock::QuantaInstant;
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::time::Duration;
use actix_web::middleware::{Compress, Logger};

use crate::service::env::EnvConfig;

pub struct Middlewares {
    pub cors: Cors,
    pub compress: Compress,
    pub logger: Logger,
    pub governor: Governor<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>,
    pub session: SessionMiddleware<CookieSessionStore>,
}

impl Middlewares {
    fn get_governor() -> Governor<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
        let governor_conf = GovernorConfigBuilder::default()
            .seconds_per_request(1)
            .burst_size(10)
            .finish()
            .expect("Governor config is not valid");

        let governor: Governor<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> =
            Governor::new(&governor_conf);

        governor
    }

    fn get_cors(envs: EnvConfig) -> Cors {
        let allowed_origins = vec![envs.fe_address.as_str()];

        let mut cors_conf = Cors::default()
            .allowed_methods(vec!["OPTIONS", "GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .expose_headers(&[
                actix_web::http::header::CONTENT_DISPOSITION,
                actix_web::http::header::SET_COOKIE,
                actix_web::http::header::HeaderName::from_static("x-success-code"),
                actix_web::http::header::HeaderName::from_static("x-error-code"),
            ])
            .supports_credentials()
            .max_age(3600);

        for origin in allowed_origins {
            cors_conf = cors_conf.allowed_origin(origin);
        }

        cors_conf
    }

    fn get_session(envs: EnvConfig) -> SessionMiddleware<CookieSessionStore> {
        let secret_key = actix_web::cookie::Key::from(envs.hmac_secret.as_bytes());

        let one_day = Duration::seconds(24 * 60 * 60); // 24 hours in seconds

        actix_session::SessionMiddleware::builder(
            actix_session::storage::CookieSessionStore::default(),
            secret_key.clone(),
        )
        .cookie_http_only(true)
        .cookie_secure(true)
        .cookie_same_site(actix_web::cookie::SameSite::Strict)
        .session_lifecycle(actix_session::config::PersistentSession::default().session_ttl(one_day))
        .cookie_path("/".to_string())
        .build()
    }

    pub fn new(envs: &EnvConfig) -> Middlewares {
        Middlewares {
            cors: Middlewares::get_cors(envs.clone()),
            compress: Compress::default(),
            logger: Logger::default(),
            governor: Middlewares::get_governor(),
            session: Middlewares::get_session(envs.clone()),
        }
    }
}
