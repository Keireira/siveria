use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct EnvConfig {
    // Base
    pub hostname: String,
    pub port: u16,
    pub fe_address: String,

    // Additional params
    pub with_migration: bool,

    // Postgres databases
    pub db_url: String,

    // Secrets
    pub secret_key: String,
    pub hmac_secret: String,

    // S3
    pub s3_bucket_name: String,
    pub s3_region: String,
    pub s3_endpoint_url: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
}

impl EnvConfig {
    pub async fn new() -> EnvConfig {
        dotenv().ok();

        let with_migration = env::var("MIGRATION").unwrap_or("false".to_string()) == "true";

        EnvConfig {
            // base params
            hostname: env::var("HOSTNAME").expect("HOSTNAME must be set"),
            port: env::var("PORT")
                .expect("PORT must be set")
                .parse::<u16>()
                .expect("PORT must be a number"),
            fe_address: env::var("FE_ADDRESS").expect("FE_ADDRESS must be set"),

            // Additional params
            with_migration: with_migration,

            // Postgres databases
            db_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),

            // Secrets
            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set"),
            hmac_secret: env::var("HMAC_SECRET").expect("HMAC_SECRET must be set"),

            // s3
            s3_bucket_name: env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set"),
            s3_region: env::var("S3_REGION").expect("S3_REGION must be set"),
            s3_endpoint_url: env::var("S3_ENDPOINT_URL").expect("S3_ENDPOINT_URL must be set"),
            s3_access_key: env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY must be set"),
            s3_secret_key: env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set"),
        }
    }
}
