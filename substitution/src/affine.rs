// 2024 Paweł Rybak

use std::error::Error;

/// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures
pub fn inverse(a: i32, n: i32) -> Option<i32> {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (n, a);

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    if r > 1 {
        return None;
    }

    if t < 0 {
        t += n;
    }

    Some(t)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn encrypt(plaintext: &str, a: i32, b: i32) -> Result<String, Box<dyn Error>> {
    if gcd(a, 26) != 1 {
        return Err("Invalid 'a' parameter (gcd(a, 26) != 1)".into());
    }

    let ciphertext: String = plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_uppercase() { b'A' } else { b'a' };
                let x = (c as u8 - base) as i32;
                (((a * x + b) % 26) as u8 + base) as char
            } else {
                c
            }
        })
        .collect();

    Ok(ciphertext)
}

pub fn decrypt(ciphertext: &str, a: i32, b: i32) -> Result<String, Box<dyn Error>> {
    if gcd(a, 26) != 1 {
        return Err("Invalid 'a' parameter (gcd(a, 26) != 1)".into());
    }

    let a_inv = inverse(a, 26).ok_or("Invalid 'a' parameter ('a' could not be inverted)")?;

    let plaintext: String = ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_uppercase() { b'A' } else { b'a' };
                let y = (c as u8 - base) as i32;
                (((a_inv * (y - b + 26)) % 26) as u8 + base) as char
            } else {
                c
            }
        })
        .collect();

    Ok(plaintext)
}
