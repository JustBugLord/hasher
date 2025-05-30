use clap::{Arg, ArgAction, Command};
use hasher::algorithms::*;
use hasher::errors::HashError;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let matches = Command::new("Hasher")
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .required(false)
                .default_value("SHA256")
                .help("Choose the hashing algorithm"),
        )
        .arg(
            Arg::new("source")
                .index(1)
                .required(true)
                .help("String/File/Directory for hashing"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Additional information")
        )
        .get_matches();
    let algorithm = match matches.get_one::<String>("algorithm") {
        Some(algorithm_name) => match HashAlgorithm::from_str(algorithm_name) {
            Ok(algorithm_real) => algorithm_real,
            Err(err) => {
                eprintln!("Fail with error: {:?}", err);
                return;
            }
        },
        _ => HashAlgorithm::SHA256,
    };
    let source = match matches.get_one::<String>("source") {
        Some(source_real) => source_real,
        _ => &String::new()
    };
    let verbose = match matches.get_one::<bool>("verbose") {
        Some(verbose_real) => verbose_real,
        _ => &false
    };

    let hash: Result<String, HashError>;

    let path = Path::new(source);
    if path.exists() {
        hash = algorithm.hash(path);
    } else {
        hash = algorithm.hash(source);
    }

    match hash {
        Ok(hash_result) => {
            if *verbose {
                println!("Algorithm: {:?} Hash: {}", algorithm, hash_result)
            } else {
                println!("{}", hash_result)
            }
        },
        Err(err) => eprintln!("Error: {:?}", err)
    }
}
