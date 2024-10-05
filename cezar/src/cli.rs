use clap::{ArgGroup, Parser};

#[derive(Debug, Parser)]
#[command(group(
    ArgGroup::new("cipher")
        .required(true)
        .args(&["caesar", "affine"]),
))]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["encrypt", "decrypt", "plaintext", "ciphertext"]),
))]
pub struct Args {
    /// Caesar cipher
    #[arg(short)]
    pub caesar: bool,

    /// Affine cipher
    #[arg(short)]
    pub affine: bool,

    /// Encryption mode
    #[arg(short)]
    pub encrypt: bool,

    /// Decryption mode
    #[arg(short)]
    pub decrypt: bool,

    /// Known-plaintext cryptanalysis
    #[arg(short = 'j')]
    pub plaintext: bool,

    /// Ciphertext cryptanalysis
    #[arg(short = 'k')]
    pub ciphertext: bool,
}
