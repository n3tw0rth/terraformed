use std::path::PathBuf;

use color_eyre::eyre::Result;
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub struct Tracer;
impl Tracer {
    pub fn init(level_filter: LevelFilter) -> Result<()> {
        // TODO: Get the data directory from CLI Args or Environment Variables
        let directory = PathBuf::from(".");
        std::fs::create_dir_all(directory.clone())?;

        // INFO: terraformed.log
        let log_path = directory.join(format!("{}.log", env!("CARGO_PKG_NAME")));
        let log_file = std::fs::File::create(log_path)?;

        let file_subscriber = tracing_subscriber::fmt::layer()
            .with_file(true)
            .with_line_number(true)
            .with_writer(log_file)
            .with_target(false)
            .with_ansi(false)
            .with_filter(level_filter);

        tracing_subscriber::registry()
            .with(file_subscriber)
            .with(ErrorLayer::default())
            .init();

        Ok(())
    }
}
