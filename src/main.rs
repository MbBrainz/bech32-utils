use bech32_utils::converter;
use clap::{error::ErrorKind, Parser};
use std::path::PathBuf;

use clap::{Args,  Subcommand};

#[derive(Parser)]
#[command(
    author = "Your Name <maurits.bos@gmail.com>",
    version = "1.0",
    about = "Transforms any bech32 prefixed address into another address with a different prefix"
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts single bech32 prefixed address into another address with a different prefix
    Single(SingleConvert),

    /// Converts a csv file with bech32 prefixed addresses into another address with a different prefix, and writes the output to a new csv file
    ConvertCsv(ConvertCsv),
}

#[derive(Args)]
struct ConvertCsv {
    /// The input csv file
    #[arg(required = true, index = 1)]
    input_dir: PathBuf, 

    /// The prefix one wants to convert it into
    #[arg(required = true, index = 2)]
    prefix: String,

    /// The output csv file
    #[arg(required = false, index = 3)]
    output_dir: Option<PathBuf>,
}
#[derive(Args)]

struct SingleConvert {
    /// The original address
    #[arg(required = true, index = 1)]
    address: String,

    /// The prefix one wants to convert it into
    #[arg(required = true, index = 2)]
    prefix: String,

    /// The optional derivation path
    #[arg(required = false, index = 3)]
    derivation_path: Option<PathBuf>,
}


fn single_convert(single: SingleConvert) {
    let address = single.address;
    let prefix = single.prefix;
    let _derivation_path = single.derivation_path;

    
    match converter::any_addr_to_prefix_addr(address, &prefix) {
        Ok(transformed_address) => println!("{}", transformed_address),
        Err(e) => {
            eprintln!("Error transforming address: {}", e);
            clap::Error::new(ErrorKind::ValueValidation);
        }
    }
}

fn convert_csv(convert_csv: ConvertCsv) {
    let input_dir = convert_csv.input_dir;

    // output_dir is optional, if not provided, the output will be written to the input file with the postfix "_converted"
    let output_dir = convert_csv.output_dir.unwrap_or_else(|| {
        add_converted_postfix(&input_dir)
    });

    let prefix = convert_csv.prefix;

    let mut rdr = csv::Reader::from_path(input_dir).unwrap();
    let mut wtr = csv::Writer::from_path(output_dir).unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let address = record.get(0).unwrap();

        // iterate over the rest of the record to get the rest of the record
        let rest = (1..record.len()).map(|i| record.get(i).unwrap())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();


        match converter::any_addr_to_prefix_addr(address.to_string(), &prefix) {
            Ok(transformed_address) => {
                wtr.write_record(&[transformed_address, rest.join(",")]).unwrap();
            }
            Err(e) => {
                eprintln!("Error transforming address: {}", e);
                clap::Error::new(ErrorKind::ValueValidation);
            }
        }
    }
    wtr.flush().unwrap();
}

fn add_converted_postfix(input_dir: &PathBuf) -> PathBuf {
    let mut output_dir = input_dir.clone();
    let file_name = output_dir.file_name().unwrap();
    let file_name = file_name.to_str().unwrap();
    let file_name = format!("{}_converted.csv", file_name);
    output_dir.set_file_name(file_name);
    output_dir
}


fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Single(single) => single_convert(single),
        Commands::ConvertCsv(command) => convert_csv(command),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_converted_postfix() {
        let input_dir = PathBuf::from("test.csv");
        let output_dir = add_converted_postfix(&input_dir);
        assert_eq!(output_dir, PathBuf::from("test_converted.csv"));
    }

    #[test]
    fn test_add_converted_postfix_with_path() {
        let input_dir = PathBuf::from("test/test.csv");
        let output_dir = add_converted_postfix(&input_dir);
        assert_eq!(output_dir, PathBuf::from("test/test_converted.csv"));
    }

}
