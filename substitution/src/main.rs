use clap::Parser;

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

fn main() {
    let args = cli::Args::parse();
    let config = parse_config(args);
    println!("{:#?}", config);
}
