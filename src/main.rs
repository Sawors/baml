use std::path::Path;

use clap::{Parser, Subcommand};

mod get;

// Current limitations :
// - cannot read multiline YAML data

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// the mode to use
    #[command(subcommand)]
    mode: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Get a value defined by a path inside a yaml file
    Get {
        /// the file to parse
        file: String,
        /// the path to the value, keys separated by a path separator ("." by default)
        data_path: String,
        /// the number of space characters used by the file for indentation, -1 for auto
        #[arg(short, long, default_value_t = -1)]
        indent: i8,
        /// a custom separator to be used by the data path
        #[arg(short, long, default_value_t = String::from("."))]
        separator: String,
        /// a default value to return if the path is not found
        #[arg(short, long)]
        default: Option<String>
    }
}

fn main() {
    let args: Args = Args::parse();
    let mode = &args.mode;
    match mode {
        Commands::Get {
            file,
            data_path,
            indent,
            separator,
            default
        } => {
            if ! Path::new(file).exists() {
                println!("File not found!");
                return
            }
            let indent_size = if indent >= &0 {*indent as usize} else {get::get_indent_size(file).unwrap_or(2)};
            let value = get::get_from_path(
                file,
                data_path,
                indent_size,
                separator
            ).or(default.clone());
            if value.is_some() {
                println!("{}", value.unwrap());
            } else {
                println!("Path not found!");
                std::process::exit(1);
            }
        }
    }
}
