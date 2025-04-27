use clap::Parser;
use tracing::level_filters::LevelFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum, default_value = "info")]
    pub log_level: Option<LevelFilter>,
}
