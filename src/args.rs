use clap::{Parser, ValueEnum};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long, default_value = "read")]
    action: Action,

    #[arg(short, long, default_value = "csv")]
    to_format: Format
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
enum Format {
    TXT,
    BIN,
    CSV
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
enum Action {
    READ,
    CONVERT
}
