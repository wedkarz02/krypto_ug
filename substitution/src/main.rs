use std::error::Error;

use clap::Parser;

mod affine;
mod caesar;
mod cli;

#[derive(Debug)]
enum Cipher {
    Caesar,
    Affine,
}

#[derive(Debug)]
enum Mode {
    Encrypt,
    Decrypt,
    Plaintext,
    Ciphertext,
}

#[allow(unused)]
#[derive(Debug)]
struct Config {
    cipher: Cipher,
    mode: Mode,
}

fn parse_config(args: cli::Args) -> Config {
    let cipher = if args.caesar {
        Cipher::Caesar
    } else {
        Cipher::Affine
    };

    let mode = if args.encrypt {
        Mode::Encrypt
    } else if args.decrypt {
        Mode::Decrypt
    } else if args.plaintext {
        Mode::Plaintext
    } else {
        Mode::Ciphertext
    };

    Config { cipher, mode }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    let config = parse_config(args);
    println!("{:#?}", config);

    let plaintext = "Lorem ipsum, kys!";
    let ciphertext = "Lapcq wfueq, gyu!";
    let key = 1;
    let a = 5;
    let b = 8;

    match (config.cipher, config.mode) {
        (Cipher::Caesar, Mode::Encrypt) => {
            let ciphertext = caesar::encrypt(plaintext, key)?;
            println!("ciphertext: {}", ciphertext);
        }
        (Cipher::Caesar, Mode::Decrypt) => {
            let plaintext = caesar::decrypt(ciphertext, key)?;
            println!("plaintext: {}", plaintext);
        }
        (Cipher::Affine, Mode::Encrypt) => {
            let ciphertext = affine::encrypt(plaintext, a, b)?;
            println!("ciphertext: {}", ciphertext);
        }
        (Cipher::Affine, Mode::Decrypt) => {
            let plaintext = affine::decrypt(ciphertext, a, b)?;
            println!("plaintext: {}", plaintext);
        }
        _ => {}
    }

    Ok(())
}
