use time;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::EnvFilter;

pub fn get_subscriber() -> impl tracing::Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info".to_string()));

    let time_format = time::format_description::parse(
        "[day] [month repr:short] [year repr:last_two] ([hour]:[minute]:[second].[subsecond digits:4])",
    )
    .expect("format string should be valid!");

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_timer(LocalTime::new(time_format))
        .finish()
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber")
}
