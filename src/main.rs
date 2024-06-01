// Requirements:
// Shall open a CSV file
// Shall create a CSV file (with headers)
// Shall parse the header
// Shall display range or selected rows
// Shall have a append option
// Shall have a small menu
// Shall have a find option
// Shall have a remove option (line number/finder)
// Shall have an insert option
// Shall take CL arguments

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct CSVCli {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    Create {},
}

fn main() {
    let cli = CSVCli::parse();

    if let Some(file_path) = cli.file.as_deref() {
        println!("Value for config: {}", file_path.display());
    }
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Create {}) => {
            println!("I am suppose to create stuff")
        }
        None => {}
    }

    println!("Hello, world!");
}
