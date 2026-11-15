#![forbid(unsafe_code)]

use std::{env, path::PathBuf};

fn main() {
    let mut args = env::args().skip(1);
    let root = match (args.next().as_deref(), args.next(), args.next()) {
        (None, None, None) => PathBuf::from("."),
        (Some("--root"), Some(path), None) => PathBuf::from(path),
        _ => {
            eprintln!("usage: apme-infra-contract-check [--root PATH]");
            std::process::exit(2);
        }
    };
    match apme_infra_contract_check::validate_repository(&root) {
        Ok(count) => println!("Rust infra contracts OK ({count} Kubernetes YAML files)"),
        Err(problems) => {
            for message in problems {
                eprintln!("error: {message}");
            }
            std::process::exit(1);
        }
    }
}
