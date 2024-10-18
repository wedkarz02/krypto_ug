// 2024 Paweł Rybak

use aes::{
    cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit},
    Aes128,
};

use crate::{pad, BLOCK_SIZE};

pub fn ecb(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128::new_from_slice(key).unwrap();
    let mut padded_plaintext = plaintext.to_vec();
    pad(&mut padded_plaintext);
    let mut ciphertext = Vec::new();

    for chunk in padded_plaintext.chunks_exact(BLOCK_SIZE) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        ciphertext.extend_from_slice(&block);
    }

    ciphertext
}
