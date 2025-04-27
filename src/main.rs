use clap::Parser;
use terraformed::{app::App, cli::Args, tracing::Tracer};
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Args::parse();
    let log_level = args.log_level;

    Tracer::init(log_level.unwrap_or(LevelFilter::INFO))?;

    tracing::info!("Application started with log level: {:?}", log_level);

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();

    result
}
