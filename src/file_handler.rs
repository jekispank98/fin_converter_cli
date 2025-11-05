use fin_converter_lib::error::ParserError;
use fin_converter_lib::handler::Parser;
use fin_converter_lib::models::financial_record::FinancialRecord;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use clap::Parser as ClapParser;
use fin_converter_lib::CsvParser;

pub fn handle_file(path: &Path) -> Result<Vec<FinancialRecord>, ParserError> {
    let file = Path::new(&path);
    let extension = file.extension();

    match extension {
        None => Err(ParserError::Format("File has no extension".to_string())),
        Some(ext) => {
            println!("Extension: {}", ext.to_str().unwrap());
            match ext.to_str().unwrap_or("").trim() {
                "csv" => {
                    let buf = read_file_to_buffer(path)?;
                    let mut csv_parser = CsvParser;
                    csv_parser.parse(buf)
                }
                "xlsx" => {
                    let buf = read_file_to_buffer(path)?;
                    let mut csv_parser = CsvParser;
                    csv_parser.parse(buf)
                }

                _ => Err(ParserError::Format(
                    "Unsupported file extension".to_string(),
                )),
            }
        }
    }
}

fn read_file_to_buffer(path: &Path) -> Result<BufReader<File>, ParserError> {
    eprintln!("Reading file: {:?}", path);
    let file = File::open(path).map_err(ParserError::Io)?;
    Ok(BufReader::new(file))
}

fn parse_csv() {}

#[derive(ClapParser, Debug)]
#[command(name = "fin-cli", version, about = "Financial formats converter")]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: PathBuf,
}