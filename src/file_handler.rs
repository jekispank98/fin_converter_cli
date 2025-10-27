use fin_converter_lib::error::ParserError;
use fin_converter_lib::handler::Parser;
use fin_converter_lib::models::csv::Csv;
use fin_converter_lib::models::financial_record::FinancialRecord;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn handle_file(path: String) -> Result<Vec<FinancialRecord>, ParserError> {
    let file = Path::new(&path);
    let extension = file.extension();

    match extension {
        None => Err(ParserError::Format("File has no extension".to_string())),
        Some(ext) => {
            println!("Extension: {}", ext.to_str().unwrap());
            match ext.to_str().unwrap_or("").trim() {
                "csv" => {
                    let buf = read_file_to_buffer(&path)?;
                    let mut csv_parser = Csv;
                    csv_parser.parse(buf)
                }
                "xlsx" => {
                    let buf = read_file_to_buffer(&path)?;
                    let mut csv_parser = Csv;
                    csv_parser.parse(buf)
                }

                _ => Err(ParserError::Format(
                    "Unsupported file extension".to_string(),
                )),
            }
        }
    }
}

fn read_file_to_buffer(path: &str) -> Result<BufReader<File>, ParserError> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(ParserError::Format(format!("Could not read file: {}", e))),
    };

    Ok(BufReader::new(file))
}

fn parse_csv() {}
