use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name="rcli",author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(long, help = "CSV has Header or not", default_value_t = true)]
    pub header: bool,
    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename.is_empty() {
        return Err("Input file is empty".to_string());
    }
    if !Path::new(filename).exists() {
        return Err("Input file does not exist".to_string());
    }
    Ok(filename.to_string())
}
