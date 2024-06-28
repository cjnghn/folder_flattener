mod cli;

use clap::Parser;
use env_logger::Env;
use log::error;

fn main() {
    let cli = cli::Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    if let Err(e) = folder_flattener::flatten_directory(&cli.source, &cli.output, cli.ignore_file.as_deref()) {
        error!("An error occurred: {}", e);
        std::process::exit(1);
    }
}