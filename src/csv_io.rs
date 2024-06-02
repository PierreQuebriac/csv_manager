use std::{error::Error, fs, path::PathBuf};

#[derive(Debug)]
pub struct CsvParams {
    with_header: bool,
    with_semi_column: bool,
}

#[derive(Debug)]
pub struct CsvData {
    params: CsvParams,
    rows: Vec<Vec<String>>,
}

pub fn parse_csv(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let read_result = fs::read_to_string(file_path)?;
    let rows: Vec<Vec<&str>> = read_result
        .lines()
        .into_iter()
        .map(|line| line.split(',').collect())
        .collect();

    println!("{rows:?}");
    Ok(())
}
