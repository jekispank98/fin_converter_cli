use clap::{Parser, ValueEnum};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: String,

    #[arg(short, long, default_value = "read")]
    pub action: Action,

    #[arg(short, long, default_value = "csv")]
    pub to_format: Format
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum Format {
    TXT,
    BIN,
    CSV
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum Action {
    READ,
    CONVERT
}
