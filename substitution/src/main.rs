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

fn run_caesar_plaintext(
    ciphertext: &str,
    plaintext: &str,
) -> Result<(i32, String), Box<dyn Error>> {
    if ciphertext.len() != plaintext.len() {
        return Err("Error: Ciphertext len != plaintext len.".into());
    }

    let mut key: Option<i32> = None;
    for (c, p) in ciphertext.chars().zip(plaintext.chars()) {
        if c.is_ascii_alphabetic() && p.is_ascii_alphabetic() {
            let base_c = if c.is_uppercase() { b'A' } else { b'a' };
            let base_p = if p.is_uppercase() { b'A' } else { b'a' };
            let pos_c = (c as u8 - base_c) as i32;
            let pos_p = (p as u8 - base_p) as i32;
            let curr_key = (pos_c - pos_p).rem_euclid(26);

            if let Some(k) = key {
                if k != curr_key {
                    return Err("Error: Invalid caesar cipher.".into());
                }
            } else {
                key = Some(curr_key);
            }
        } else if c != p {
            return Err("Error: Ciphertext char type != plaintext char type.".into());
        }
    }

    let key = key.ok_or("Error: Unable to find the key.")?;
    let decrypted_text = caesar::decrypt(ciphertext, key)?;

    Ok((key, decrypted_text))
}

fn run_caesar_ciphertext() -> Result<(), Box<dyn Error>> {
    let ciphertext = match read_to_string("crypto.txt") {
        Ok(msg) => msg,
        Err(_) => {
            eprintln!("Error: 'crypto.txt' file not found.");
            process::exit(0);
        }
    };
    let mut all_cases = String::new();
    for key in 1..=25 {
        let decrypted_text = caesar::decrypt(&ciphertext, key)?;
        all_cases.push_str(&format!("Key {}: {}\n", key, decrypted_text));
    }
    let mut plain_file = File::create("plain.txt")?;
    plain_file.write_all(all_cases.as_bytes())?;
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

fn run_affine_plaintext(ciphertext: &str, plaintext: &str) -> Result<(i32, i32), Box<dyn Error>> {
    if ciphertext.len() < 2 || plaintext.len() < 2 {
        return Err("Error: 2 or more character pairs needed.".into());
    }

    let (x1, x2) = (
        plaintext.chars().nth(0).unwrap(),
        plaintext.chars().nth(1).unwrap(),
    );
    let (y1, y2) = (
        ciphertext.chars().nth(0).unwrap(),
        ciphertext.chars().nth(1).unwrap(),
    );

    let base_x1 = if x1.is_uppercase() { b'A' } else { b'a' };
    let base_y1 = if y1.is_uppercase() { b'A' } else { b'a' };
    let base_x2 = if x2.is_uppercase() { b'A' } else { b'a' };
    let base_y2 = if y2.is_uppercase() { b'A' } else { b'a' };

    let x1_pos = (x1 as u8 - base_x1) as i32;
    let y1_pos = (y1 as u8 - base_y1) as i32;
    let x2_pos = (x2 as u8 - base_x2) as i32;
    let y2_pos = (y2 as u8 - base_y2) as i32;

    let diff_x = (x1_pos - x2_pos).rem_euclid(26);
    let diff_y = (y1_pos - y2_pos).rem_euclid(26);

    let a = affine::inverse(diff_x, 26)
        .ok_or("Invalid 'a' parameter ('a' could not be inverted)")?
        * diff_y;

    let a = a.rem_euclid(26);
    let b = (y1_pos - a * x1_pos).rem_euclid(26);
    Ok((a, b))
}

fn run_affine_ciphertext() -> Result<(), Box<dyn Error>> {
    let ciphertext = match read_to_string("crypto.txt") {
        Ok(msg) => msg,
        Err(_) => {
            eprintln!("Error: 'crypto.txt' file not found.");
            process::exit(0);
        }
    };
    let mut all_cases = String::new();
    for a in 1..=25 {
        for b in 0..=25 {
            if let Ok(decrypted_text) = affine::decrypt(&ciphertext, a, b) {
                all_cases.push_str(&format!("Key (a={}, b={}): {}\n", a, b, decrypted_text));
            }
        }
    }
    let mut plain_file = File::create("plain.txt")?;
    plain_file.write_all(all_cases.as_bytes())?;
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
        (Cipher::Caesar, Mode::Plaintext) => {
            let ciphertext = match read_to_string("crypto.txt") {
                Ok(msg) => msg,
                Err(_) => {
                    eprintln!("Error: 'crypto.txt' file not found.");
                    process::exit(0);
                }
            };
            let plaintext = match read_to_string("extra.txt") {
                Ok(msg) => msg,
                Err(_) => {
                    eprintln!("Error: 'extra.txt' file not found.");
                    process::exit(0);
                }
            };
            let res = run_caesar_plaintext(&ciphertext, &plaintext)?;
            let mut key_new_file = File::create("key-new.txt")?;
            key_new_file.write_all(res.0.to_string().as_bytes())?;
            let mut decrypt_file = File::create("decrypt.txt")?;
            decrypt_file.write_all(res.1.as_bytes())?;
        }
        (Cipher::Caesar, Mode::Ciphertext) => {
            run_caesar_ciphertext()?;
        }
        (Cipher::Affine, Mode::Encrypt) => {
            run_affine_encrypt()?;
        }
        (Cipher::Affine, Mode::Decrypt) => {
            run_affine_decrypt()?;
        }
        (Cipher::Affine, Mode::Plaintext) => {
            let ciphertext = match read_to_string("crypto.txt") {
                Ok(msg) => msg,
                Err(_) => {
                    eprintln!("Error: 'crypto.txt' file not found.");
                    process::exit(0);
                }
            };
            let plaintext = match read_to_string("extra.txt") {
                Ok(msg) => msg,
                Err(_) => {
                    eprintln!("Error: 'extra.txt' file not found.");
                    process::exit(0);
                }
            };
            let keys = run_affine_plaintext(&ciphertext, &plaintext)?;
            let decrypted_text = affine::decrypt(&ciphertext, keys.0, keys.1)?;
            let mut key_new_file = File::create("key-new.txt")?;
            key_new_file.write_all(format!("{} {}\n", keys.0, keys.1).as_bytes())?;
            let mut decrypt_file = File::create("decrypt.txt")?;
            decrypt_file.write_all(decrypted_text.as_bytes())?;
        }
        (Cipher::Affine, Mode::Ciphertext) => {
            run_affine_ciphertext()?;
        }
    }

    Ok(())
}
