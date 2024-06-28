use clap::Parser;
use env_logger::Env;
use log::{error, info};
use std::process;
use std::path::Path;

mod cli;

use folder_flattener::flatten_directory;

fn main() {
    let cli = cli::Cli::parse();

    // Initialize the logger
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .init();

    // Set the log level based on verbosity
    match cli.verbose {
        0 => log::set_max_level(log::LevelFilter::Info),
        1 => log::set_max_level(log::LevelFilter::Debug),
        _ => log::set_max_level(log::LevelFilter::Trace),
    }

    info!("Starting Folder Flattener");
    info!("Source directory: {}", cli.source);
    info!("Output directory: {}", cli.output);

    let ignore_file = Path::new(&cli.source).join(".flatternignore");
    let ignore_file = ignore_file.to_str();

    match flatten_directory(&cli.source, &cli.output, ignore_file) {
        Ok(_) => info!("Folder flattening completed successfully"),
        Err(e) => {
            error!("An error occurred while flattening the directory: {}", e);
            process::exit(1);
        }
    }
}