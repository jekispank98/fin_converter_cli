use crate::args::{Action, Args, Format};
use fin_converter_lib::error::ParserError;
use fin_converter_lib::handler::{Parser, Serializer};
use fin_converter_lib::models::bin::Bin;
use fin_converter_lib::models::csv::Csv;
use fin_converter_lib::models::text::Txt;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

pub fn resolve_action(args: Args) {
    let path = normalize_path(&args.path);
    let to_format = args.to_format;

    if !is_file_exist(&path) {
        eprintln!("Error: File '{}' does not exist", path.display());
        return;
    }

    println!("Processing file: {}", path.display());
    println!("Action: {:?}, Target format: {:?}", args.action, &to_format);

    // Route based on action
    match args.action {
        Action::READ => {
            if let Err(e) = read_action(&path) {
                eprintln!("Error during read action: {}", e);
            }
        }
        Action::CONVERT => {
            if let Err(e) = convert_action(&path, &to_format) {
                eprintln!("Error during convert action: {}", e);
            }
        }
    }
}

fn normalize_path(raw: &str) -> PathBuf {
    let trimmed = raw.trim();
    let no_quotes = trimmed
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or(trimmed);
    PathBuf::from(no_quotes)
}

fn is_file_exist(path: &PathBuf) -> bool {
    Path::new(path).exists()
}

fn convert_action(path: &Path, target_format: &Format) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Performing CONVERT action on: {} to {:?}",
        path.display(),
        target_format
    );

    match target_format {
        Format::TXT => convert_to_txt(path)?,
        Format::BIN => convert_to_bin(path)?,
        Format::CSV => convert_to_csv(path)?,
    }

    println!("Conversion to {:?} completed successfully", target_format);
    Ok(())
}

fn read_action(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let buf = read_file_to_buffer(path)?;

    match extension {
        "csv" => {
            let mut csv_parser = Csv;
            csv_parser.parse(buf)?;
        }
        "txt" => {
            let mut txt_parser = Txt;
            txt_parser.parse(buf)?;
        }
        "bin" => {
            let mut bin_parser = Bin;
            bin_parser.parse(buf)?;
        }
        "" => return Err(ParserError::Format("нет расширения".into()).into()),
        _ => return Err(ParserError::Format("неподдерживаемое расширение".into()).into()),
    }
    Ok(())
}

fn convert_to_txt(path: &Path) -> Result<(), ParserError> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let buf = read_file_to_buffer(path)?;

    let out_path = path.with_extension("txt");
    let out_file = File::create(&out_path).map_err(ParserError::Io)?;
    let writer = BufWriter::new(out_file);
    match extension {
        "txt" => {
            println!("The file is TXT already!");
            Ok(())
        }
        "csv" => {
            let mut csv_parser = Csv;
            let records = csv_parser.parse(buf)?;

            let txt_serializer = Txt;
            txt_serializer.serialize(&records, writer)
        }
        "bin" => {
            let mut bin_parser = Bin;
            let records = bin_parser.parse(buf)?;

            let txt_serializer = Txt;
            txt_serializer.serialize(&records, writer)
        }
        _ => {
            println!("Unexpected file's format!");
            Ok(())
        }
    }
}

fn convert_to_bin(path: &Path) -> Result<(), ParserError> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let buf = read_file_to_buffer(path)?;

    let out_path = path.with_extension("bin");
    let out_file = File::create(&out_path).map_err(ParserError::Io)?;
    let writer = BufWriter::new(out_file);
    match extension {
        "txt" => {

            let mut txt_parser = Txt;
            let records = txt_parser.parse(buf)?;

            let bin_serializer = Bin;
            bin_serializer.serialize(&records, writer)
        }
        "csv" => {
            let mut csv_parser = Csv;
            let records = csv_parser.parse(buf)?;

            let bin_serializer = Bin;
            bin_serializer.serialize(&records, writer)
        }
        "bin" => {
            println!("The file is BIN already!");
            Ok(())
        }
        _ => {
            println!("Unexpected file's format!");
            Ok(())
        }
    }
}
fn convert_to_csv(path: &Path) -> Result<(), ParserError> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let buf = read_file_to_buffer(path)?;

    let out_path = path.with_extension("csv");
    let out_file = File::create(&out_path).map_err(ParserError::Io)?;
    let writer = BufWriter::new(out_file);
    match extension {
        "txt" => {
            let mut txt_parser = Txt;
            let records = txt_parser.parse(buf)?;

            let csv_serializer = Csv;
            csv_serializer.serialize(&records, writer)
        }
        "csv" => {
            println!("The file is BIN already!");
            Ok(())
        }
        "bin" => {
            let mut bin_parser = Bin;
            let records = bin_parser.parse(buf)?;

            let csv_serializer = Csv;
            csv_serializer.serialize(&records, writer)
        }
        _ => {
            println!("Unexpected file's format!");
            Ok(())
        }
    }
}

fn read_file_to_buffer(path: &Path) -> Result<BufReader<File>, ParserError> {
    eprintln!("Reading file: {:?}", path);
    let file = File::open(path).map_err(ParserError::Io)?;
    Ok(BufReader::new(file))
}

/*
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
                    let mut csv_parser = Csv;
                    csv_parser.parse(buf)
                }
                "txt" => {
                    let buf = read_file_to_buffer(path)?;
                    let mut txt_parser = Txt;
                    txt_parser.parse(buf)
                }
                "bin" => {
                    let buf = read_file_to_buffer(path)?;
                    let mut bin_parser = Bin;
                    bin_parser.parse(buf)
                }

                _ => Err(ParserError::Format(
                    "Unsupported file extension".to_string(),
                )),
            }
        }
    }
}

fn parse_csv() {}

#[derive(ClapParser, Debug)]
#[command(name = "fin-cli", version, about = "Financial formats converter")]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: PathBuf,
}*/
