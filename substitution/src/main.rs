// 2024 Paweł Rybak

use std::{
    error::Error,
    fs::{read_to_string, File},
    io::Write,
    process,
};

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

fn key_tuple() -> Result<(i32, i32), Box<dyn Error>> {
    let keys = match read_to_string("key.txt") {
        Ok(msg) => msg,
        Err(_) => {
            eprintln!("Error: 'key.txt' file not found.");
            process::exit(0);
        }
    };
    let mut keys = keys.split_ascii_whitespace();
    let a = keys.next();
    let b = keys.next();
    let (a, b) = match (a, b) {
        (Some(aa), Some(bb)) => match (aa.parse(), bb.parse()) {
            (Ok(aaa), Ok(bbb)) => (aaa, bbb),
            _ => {
                eprintln!("Error: Invalid key.");
                process::exit(0);
            }
        },
        _ => {
            eprintln!("Error: Invalid key.");
            process::exit(0);
        }
    };
    Ok((a, b))
}

fn run_caesar_encrypt() -> Result<(), Box<dyn Error>> {
    let (key, _) = key_tuple()?;
    let plaintext = match read_to_string("plain.txt") {
        Ok(msg) => msg,
        Err(_) => {
            eprintln!("Error: 'plain.txt' file not found.");
            process::exit(0);
        }
    };
    let ciphertext = caesar::encrypt(&plaintext, key)?;
    let mut crypto_file = File::create("crypto.txt")?;
    crypto_file.write_all(ciphertext.as_bytes())?;
    Ok(())
}

fn run_caesar_decrypt() -> Result<(), Box<dyn Error>> {
    let (key, _) = key_tuple()?;
    let ciphertext = match read_to_string("crypto.txt") {
        Ok(msg) => msg,
        Err(_) => {
            eprintln!("Error: 'crypto.txt' file not found.");
            process::exit(0);
        }
    };
    let plaintext = caesar::decrypt(&ciphertext, key)?;
    let mut decrypt_file = File::create("decrypt.txt")?;
    decrypt_file.write_all(plaintext.as_bytes())?;
    Ok(())
}

fn run_affine_encrypt() -> Result<(), Box<dyn Error>> {
    let (a, b) = key_tuple()?;
    let plaintext = match read_to_string("plain.txt") {
        Ok(msg) => msg,
        Err(_) => {
            eprintln!("Error: 'plain.txt' file not found.");
            process::exit(0);
        }
    };
    let ciphertext = affine::encrypt(&plaintext, a, b)?;
    let mut crypto_file = File::create("crypto.txt")?;
    crypto_file.write_all(ciphertext.as_bytes())?;
    Ok(())
}

fn run_affine_decrypt() -> Result<(), Box<dyn Error>> {
    let (a, b) = key_tuple()?;
    let ciphertext = match read_to_string("crypto.txt") {
        Ok(msg) => msg,
        Err(_) => {
            eprintln!("Error: 'crypto.txt' file not found.");
            process::exit(0);
        }
    };
    let plaintext = affine::decrypt(&ciphertext, a, b)?;
    let mut decrypt_file = File::create("decrypt.txt")?;
    decrypt_file.write_all(plaintext.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    let config = parse_config(args);

    match (config.cipher, config.mode) {
        (Cipher::Caesar, Mode::Encrypt) => {
            run_caesar_encrypt()?;
        }
        (Cipher::Caesar, Mode::Decrypt) => {
            run_caesar_decrypt()?;
        }
        (Cipher::Caesar, Mode::Plaintext) => {}
        (Cipher::Caesar, Mode::Ciphertext) => {}
        (Cipher::Affine, Mode::Encrypt) => {
            run_affine_encrypt()?;
        }
        (Cipher::Affine, Mode::Decrypt) => {
            run_affine_decrypt()?;
        }
        (Cipher::Affine, Mode::Plaintext) => {}
        (Cipher::Affine, Mode::Ciphertext) => {}
    }

    Ok(())
}
