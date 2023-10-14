use tracing::subscriber::set_global_default;
use tracing_subscriber::{
    filter::EnvFilter, prelude::__tracing_subscriber_SubscriberExt, Registry,
};

pub fn telemetry() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error"));
    set_global_default(
        Registry::default()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().pretty()),
    )
    .ok();
}
