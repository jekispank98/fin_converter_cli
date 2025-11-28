use crate::args::{Action, Args, Format};
use chrono::{DateTime, Local, TimeZone};
use fin_converter_lib::error::ParserError;
use fin_converter_lib::handler::{Parser, Serializer};
use fin_converter_lib::models::bin::Bin;
use fin_converter_lib::models::csv::Csv;
use fin_converter_lib::models::financial_record::FinancialRecord;
use fin_converter_lib::models::text::Txt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

pub fn resolve_action(args: Args) {
    let path = normalize_path(&args.path);
    let path_to_compare = normalize_path(&args.path_to_compare);
    let to_format = args.to_format;

    eprintln!("path '{:?}' ", path);
    if !is_file_exist(&PathBuf::from(&args.path)) {
        eprintln!("Error: File '{}' does not exist", path.display());
        return;
    }

    println!("Processing file: {}", path.display());
    println!("Action: {:?}, Target format: {:?}", args.action, &to_format);

    match args.action {
        Action::READ => match read_action(&path) {
            Ok(list) => list.iter().for_each(|r| print_one_record(r)),
            Err(e) => {
                eprintln!("Error during read action: {:?}", e);
            }
        },
        Action::CONVERT => {
            if let Err(e) = convert_action(&path, &to_format) {
                eprintln!("Error during convert action: {:?}", e);
            }
        }
        Action::COMPARE => {
            if path_to_compare.as_path().as_os_str().is_empty() {
                eprintln!("Undefined path! Check both paths and try again!")
            } else {
                let compared = compare_files(&path, &path_to_compare);
                match compared {
                    Ok(list) => {
                        if list.is_empty() {
                            println!("Files are identical!")
                        } else {
                            list.iter().for_each(|record| print_one_record(record));
                        }
                    }
                    Err(e) => {
                        eprintln!("Error during comparable action: {:?}", e);
                    }
                }
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
    println!("Проверяем путь: {}", path.display());

    let exists = path.exists();
    let is_file = path.is_file();

    println!("exists: {}, is_file: {}", exists, is_file);

    exists && is_file
}

fn convert_action(path: &Path, target_format: &Format) -> Result<(), ParserError> {
    println!(
        "Performing CONVERT action on: {} to {:?}",
        path.display(),
        target_format
    );

    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let output_file_extension = target_format.to_string().to_lowercase();
    let buf = read_file_to_buffer(path)?;
    let out_path = path.with_extension(output_file_extension);
    let out_file = File::create(&out_path).map_err(ParserError::Io)?;
    let writer = BufWriter::new(out_file);
    let parse_result: Result<Format, _> = extension.parse();
    let _format = match parse_result {
        Ok(format) => {
            let records = match format {
                Format::TXT => Txt::parse(&mut Txt, buf),
                Format::BIN => Bin::parse(&mut Bin, buf),
                Format::CSV => Csv::parse(&mut Csv, buf),
            };

            match records {
                Ok(r) => {
                    let _serializer = match target_format {
                        Format::TXT => Txt::serialize(&Txt, &r, writer),
                        Format::BIN => Bin::serialize(&Bin, &r, writer),
                        Format::CSV => Csv::serialize(&Csv, &r, writer),
                    };
                }
                Err(e) => {
                    eprintln!("Error during conversion action: {:?}", e);
                }
            }
        }
        Err(_) => {
            return Err(ParserError::Format(String::from(
                "Unexpected format extension!",
            )));
        }
    };
    println!("Conversion to {:?} completed successfully", target_format);
    Ok(())
}

fn read_action(path: &Path) -> Result<Vec<FinancialRecord>, ParserError> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let buf = read_file_to_buffer(path)?;

    match extension {
        "csv" => {
            let mut csv_parser = Csv;
            Ok(csv_parser.parse(buf)?)
        }
        "txt" => {
            let mut txt_parser = Txt;
            Ok(txt_parser.parse(buf)?)
        }
        "bin" => {
            let mut bin_parser = Bin;
            Ok(bin_parser.parse(buf)?)
        }
        "" => Err(ParserError::Format("No extension".into())),
        _ => Err(ParserError::Format("Unsupported extension".into()).into()),
    }
}

fn compare_files(
    first_path: &Path,
    second_path: &Path,
) -> Result<Vec<FinancialRecord>, ParserError> {
    let first_vec = read_action(first_path)?;
    let second_vec = read_action(second_path)?;

    let map2: HashMap<i64, &FinancialRecord> = second_vec.iter().map(|r| (r.tx_id, r)).collect();

    let result: Vec<FinancialRecord> = first_vec
        .iter()
        .filter(|record1| {
            if let Some(record2) = map2.get(&record1.tx_id) {
                record1.tx_type != record2.tx_type
                    || record1.from_user_id != record2.from_user_id
                    || record1.to_user_id != record2.to_user_id
                    || record1.amount != record2.amount
                    || record1.timestamp != record2.timestamp
                    || record1.status != record2.status
                    || record1.description != record2.description
            } else {
                false
            }
        })
        .cloned()
        .collect();
    Ok(result)
}
fn read_file_to_buffer(path: &Path) -> Result<BufReader<File>, ParserError> {
    eprintln!("Reading file: {:?}", path);
    let file = File::open(path).map_err(ParserError::Io)?;
    Ok(BufReader::new(file))
}

fn print_one_record(record: &FinancialRecord) {
    let pattern = format!(
        "Id: {}\nTx_type: {}\nFrom_user: {}\nTo_user: {}\nTimestamp: {}\nAmount: {}\nStatus: {}\nDescription: {}\n",
        record.tx_id,
        record.tx_type,
        record.from_user_id,
        record.to_user_id,
        format_timestamp_millis(record.timestamp),
        record.amount,
        record.status,
        record.description
    );
    println!("{}", pattern)
}

fn format_timestamp_millis(ts_millis: i64) -> String {
    let datetime: DateTime<Local> = Local
        .timestamp_millis_opt(ts_millis)
        .single()
        .expect("invalid timestamp");

    datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}
