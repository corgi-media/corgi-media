pub mod config;
pub mod constant;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}
