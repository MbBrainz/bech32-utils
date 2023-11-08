use bech32_utils::converter;
use clap::{error::ErrorKind, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author = "Your Name <maurits.bos@gmail.com>",
    version = "1.0",
    about = "Transforms any bech32 prefixed address into another address with a different prefix"
)]
struct Cli {
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

fn main() {
    let cli = Cli::parse();

    match converter::any_addr_to_prefix_addr(cli.address, &cli.prefix) {
        Ok(transformed_address) => println!("{}", transformed_address),
        Err(e) => {
            eprintln!("Error transforming address: {}", e);
            clap::Error::new(ErrorKind::ValueValidation);
        }
    }
}
