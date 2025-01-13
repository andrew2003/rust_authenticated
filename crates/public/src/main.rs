
use clap::{Parser, Subcommand};

use public::options::Options;
use rust_authenticated_common::cli_args::CliArgs;
use rust_authenticated_common::loggers::telemetry::init_telemetry;

#[tokio::main]
async fn main() {
    let options: Options = CliArgs::default_run_or_get_options(env!("APP_VERSION"));
    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
        options.log.level.as_str(),
    );
}

/// Rust Authenticated Rest API.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Config file
    #[arg(short, long, default_value = "config/00-default.toml")]
    config_path: Vec<String>,
    /// Print version
    #[clap(short, long)]
    version: bool,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// Print config
    Config,
}