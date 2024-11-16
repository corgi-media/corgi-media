use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub use tracing::*;

pub fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(Level::TRACE.into())
                .with_env_var("CORGI_LOG_LEVEL")
                .from_env_lossy(),
        )
        .init();
}
