// 2024 Paweł Rybak

use aes::{
    cipher::{BlockEncrypt, KeyInit},
    Aes128,
};

use crate::{pad, BLOCK_SIZE};

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

pub fn cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes128::new_from_slice(key).unwrap();
    let mut padded_plaintext = plaintext.to_vec();
    pad(&mut padded_plaintext);
    let mut ciphertext = Vec::new();
    let mut previous_block = iv.to_vec();

    for chunk in padded_plaintext.chunks_exact(BLOCK_SIZE) {
        let xored_block = xor(chunk, &previous_block);
        let mut block_array = [0u8; BLOCK_SIZE];
        block_array.copy_from_slice(&xored_block);
        cipher.encrypt_block(&mut block_array.into());
        ciphertext.extend_from_slice(&block_array);
        previous_block = block_array.to_vec();
    }

    ciphertext
}
