// 2024 Paweł Rybak

use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, Write},
    path::Path,
};

use clap::Parser;
use cli::Args;

mod cli;

#[derive(Debug)]
enum Mode {
    Prepare,
    Encrypt,
    Cryptanalysis,
}

#[derive(Debug)]
struct Config(Mode);

fn prepare_text() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("orig.txt")
        .map_err(|_| "Error: File 'orig.txt' not found.")?
        .replace("\n", " ");
    let text = text_cleanup(text);
    let max_len = 64;
    let mut ctr = 0;

    for _ in text.as_bytes().chunks(max_len) {
        ctr += 1;
    }

    if ctr < 2 {
        return Err("Error: Plaintext is too short for the cryptanalysis.".into());
    }

    let mut plain_file = File::create("plain.txt")?;
    for chunk in text.as_bytes().chunks(max_len) {
        let line = String::from_utf8_lossy(chunk);
        writeln!(plain_file, "{:width$}", line, width = max_len)?;
    }

    Ok(())
}

fn text_cleanup(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || *c == ' ')
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn read_lines<P>(file_name: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn read_chunks<P>(file_name: P, chunk_size: usize) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let text = fs::read_to_string(file_name)?;
    let mut chunks = Vec::new();

    for chunk in text.as_bytes().chunks(chunk_size) {
        let line = String::from_utf8_lossy(chunk).to_string();
        if line.len() != 64 {
            continue;
        }
        chunks.push(line);
    }

    Ok(chunks)
}

fn stream(text: &str, key: &str) -> String {
    text.chars()
        .zip(key.chars())
        .map(|(t, k)| (t as u8 ^ k as u8) as char)
        .collect()
}

fn encrypt() -> Result<(), Box<dyn Error>> {
    let plain_lines = read_lines("plain.txt").map_err(|_| "Error: File 'plain.txt' not found.")?;
    println!("{:#?}", plain_lines);
    let key = fs::read_to_string("key.txt")
        .map_err(|_| "Error: File 'key.txt' not found.")?
        .trim()
        .to_string();

    if key.len() != plain_lines[0].len() {
        return Err("Error: Key length must be the same as plain lines length.".into());
    }

    let mut crypto_file = File::create("crypto.txt")?;
    for line in plain_lines {
        let encrypted_line = stream(&line, &key);
        writeln!(crypto_file, "{}", encrypted_line)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mode = if args.prepare {
        Mode::Prepare
    } else if args.encrypt {
        Mode::Encrypt
    } else {
        Mode::Cryptanalysis
    };

    let config = Config(mode);

    match config.0 {
        Mode::Prepare => {
            if let Err(e) = prepare_text() {
                println!("{}", e);
            }
        }
        Mode::Encrypt => {
            if let Err(e) = encrypt() {
                println!("{}", e);
            }
        }
        Mode::Cryptanalysis => {
            // if let Err(e) = cryptanalysis() {
            //     println!("{}", e);
            // }
        }
    }

    Ok(())
}
