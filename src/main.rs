use std::{error::Error, path::PathBuf};

pub mod csv_io;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct CSVCli {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Create {},
    Update {},
    Insert {},
    Delete {},
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CSVCli::parse();

    if let Some(file_path) = cli.file.as_deref() {
        if !file_path.exists() {
            return Err(Box::<dyn Error>::from(format!(
                "The file: {} does not exist",
                file_path.display()
            )));
        }

        csv_io::parse_csv(file_path.to_path_buf())?;
    }

    match &cli.command {
        Some(Commands::Create {}) => {
            println!("I am suppose to create stuff")
        }
        None => {}
        _ => {
            println!("Unknown command")
        }
    }

    Ok(())
}
