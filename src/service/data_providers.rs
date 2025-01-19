use crate::service::env::EnvConfig;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

pub struct WebDataPool {
    pub pg: r2d2::Pool<ConnectionManager<PgConnection>>,
    pub s3: Box<s3::Bucket>,
    pub envs: EnvConfig,
}

impl WebDataPool {
    fn create_db_pool(envs: &EnvConfig) -> r2d2::Pool<ConnectionManager<PgConnection>> {
        let manager = ConnectionManager::<PgConnection>::new(&envs.db_url);

        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        tracing::event!(target: "[WEB DATA POOL]", tracing::Level::INFO, "Postgres connection pool created");

        pool
    }

    async fn create_s3_pool(envs: &EnvConfig) -> Box<s3::Bucket> {
        let base_url = envs.s3_endpoint_url.clone();
        let access_key = envs.s3_access_key.clone();
        let secret_key = envs.s3_secret_key.clone();
        let bucket_name = envs.s3_bucket_name.clone();
        let region = envs.s3_region.to_string();

        let region = Region::Custom {
            region: region.to_owned(),
            endpoint: base_url.to_owned(),
        };

        let credentials = Credentials::new(
            Some(access_key.as_str()),
            Some(secret_key.as_str()),
            None,
            None,
            None,
        )
        .expect("Should be successful");

        let bucket = Bucket::new(&bucket_name, region.clone(), credentials.clone())
            .expect("Should be successful")
            .with_path_style();

        tracing::event!(target: "[WEB DATA POOL]", tracing::Level::INFO, "S3 connection pool created");

        bucket
    }

    pub async fn new(envs: &EnvConfig) -> WebDataPool {
        WebDataPool {
            pg: WebDataPool::create_db_pool(&envs),
            s3: WebDataPool::create_s3_pool(&envs).await,
            envs: {
                tracing::event!(target: "[WEB DATA POOL]", tracing::Level::INFO, "Envs connection pool created");

                envs.clone()
            },
        }
    }
}
