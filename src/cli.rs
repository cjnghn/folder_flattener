use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Source directory to flatten
    #[arg(short, long)]
    pub source: String,

    /// Output directory for flattened files
    #[arg(short, long)]
    pub output: String,

    /// Verbose mode (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}