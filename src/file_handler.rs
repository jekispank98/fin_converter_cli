use std::ffi::OsStr;
use std::path::Path;
use fin_converter_lib::models::csv::Csv;


pub fn handle_file(path: String) {
    let file = Path::new(&path);
    let extension = file.extension();
    
    match extension {
        None => {}
        Some(ext) => {
            println!("Extension: {}", ext.to_str().unwrap());
        }
    }
}

fn read_file_toBuffer(path: &str) {
    
}

fn parse_csv() {
    
}