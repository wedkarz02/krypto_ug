use std::error::Error;

pub fn encrypt(plaintext: &str, key: i32) -> Result<String, Box<dyn Error>> {
    if !(1..=25).contains(&key) {
        return Err("Invalid key (0 <= key <= 25)".into());
    }

    let ciphertext: String = plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_uppercase() { b'A' } else { b'a' };
                (((c as u8 - base + key as u8) % 26) + base) as char
            } else {
                c
            }
        })
        .collect();

    Ok(ciphertext)
}

pub fn decrypt(ciphertext: &str, key: i32) -> Result<String, Box<dyn Error>> {
    encrypt(ciphertext, 26 - key)
}
