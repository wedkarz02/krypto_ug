// 2024 Paweł Rybak

use clap::{ArgGroup, Parser};

#[derive(Debug, Parser)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["prepare", "encrypt", "cryptanalysis"]),
))]
pub struct Args {
    /// Prepare the original plaintext
    #[arg(short)]
    pub prepare: bool,

    /// Encryption mode
    #[arg(short)]
    pub encrypt: bool,

    /// Ciphertext cryptanalysis
    #[arg(short = 'k')]
    pub cryptanalysis: bool,
}
