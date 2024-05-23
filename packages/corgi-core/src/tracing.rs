use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use tracing::*;

pub fn with_default<T>(f: impl FnOnce() -> T) -> T {
    let subscriber = tracing_subscriber::fmt().finish();

    let _guard = tracing::subscriber::set_default(subscriber);

    f()
}

pub fn init() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}
