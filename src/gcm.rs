extern crate rand;

use crate::file_handling::{read_file_into_byte_vec,  write_file_from_u8_byte_vec, 
                           write_file_from_u64_byte_vec, iv_to_bytes, 
                           read_hex_file_into_u8_byte_vec, read_hex_file_into_u64_byte_vec,
                           write_deciphered_text_from_u8_byte_vec};
use crate::speck::{bytes_to_words, encrypt, expand_keys, words_to_bytes};
use crate::handle_random::{generate_counter, generate_key};
use crate::utils::concatenate;




pub fn encrypt_to_file() {
    // Get plaintext from file and convert to byte vec
    let plaintext_bytes: Vec<u8> = read_file_into_byte_vec("text_files/plaintext.txt");

    // Split text into full blocks that can be quickly xord with encrypted counter as u64 words
    let num_full_blocks: usize = plaintext_bytes.len() / 16;
    let plaintext_full_blocks: Vec<u8> = (&plaintext_bytes[0..num_full_blocks * 16]).to_vec();

    // Get remainder of the plaintext as a u8 vector for partial xor with counter
    let plaintext_remainder: Vec<u8> = (&plaintext_bytes[num_full_blocks * 16..plaintext_bytes.len()]).to_vec();
    
    // Function converts words into litte endian order bytes
    let plaintext_words: Vec<u64> = bytes_to_words(plaintext_full_blocks);

    // Get authtext from file and convert to byte vec

    // Generate random key
    let keys: Vec<u64> = generate_key();
    let expanded_keys: [u64; 34] = expand_keys(&keys);
    // Create H value by encrypting zero array
    // let zero: u64 = 0;
    // let h: (u64, u64) = encrypt(zero, zero, &expanded_keys);
    // Generate counter (includes random IV)
    let counter: Vec<u32> = generate_counter();

    // Create ciphertext vector
    let mut ciphertext_blocks: Vec<u64> = Vec::new();
    let mut ciphertext_remainder: Vec<u8> = Vec::new();

    // Perform full encryption of plaintext
    // LATER I NEED to ensure the number of plaintext blocks does not e
    // exceed 32 bits of memory
    // Turns 4 u32 chunks into 2 u64 chunks. Preserves order.
    let (concat_counter_0, concat_counter_1) = concatenate(&counter);

    // We want to save the original latter end of the counter for authentication but also
    // increment it for encryption
    let mut incrementable_counter_1: u64 = concat_counter_1;
    for i in (0..plaintext_words.len()).step_by(2) {
        let (enc_counter_1, enc_counter_0) = encrypt(incrementable_counter_1, concat_counter_0, &expanded_keys);
        incrementable_counter_1 += 1;
        ciphertext_blocks.push(enc_counter_0 ^ plaintext_words[i]);
        ciphertext_blocks.push(enc_counter_1 ^ plaintext_words[i + 1]);
    }
    // Encrypt the remaining plaintext bits
    let (enc_counter_1, enc_counter_0) = encrypt(incrementable_counter_1, concat_counter_0, &expanded_keys);
    let enc_counter_rem_0: [u8; 8] = enc_counter_0.to_le_bytes();
    let enc_counter_rem_1: [u8; 8] = enc_counter_1.to_le_bytes();
    for i in 0..plaintext_remainder.len() {
        if i < 8 {
            ciphertext_remainder.push(plaintext_remainder[i] ^ enc_counter_rem_0[i]);
        } else{
            ciphertext_remainder.push(plaintext_remainder[i] ^ enc_counter_rem_1[i % 8]);
        }
    }
    // Create tag with GHASH function

    // Write IV, ciphertext, tag to ciphertext.txt
    let mut ciphertext_file_bytes: Vec<u8> = iv_to_bytes(&counter);
    ciphertext_file_bytes.extend_from_slice(&words_to_bytes(&ciphertext_blocks));
    ciphertext_file_bytes.extend_from_slice(&ciphertext_remainder);
    if let Err(e) = write_file_from_u8_byte_vec("text_files/ciphertext.txt", ciphertext_file_bytes) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Cipher values written to {}", "text_files/ciphertext.txt");
    }
    if let Err(e) = write_file_from_u64_byte_vec("text_files/keys.txt", keys) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Key Hex values written to {}", "text_files/keys.txt");
    }
}
pub fn decrypt_to_file() {
    // Read ciphertext from hex file
    let mut ciphertext_bytes: Vec<u8> = read_hex_file_into_u8_byte_vec("text_files/ciphertext.txt");
    // Seperate IV from ciphertext
    let mut iv: Vec<u8> = Vec::with_capacity(12);
    iv.extend_from_slice(&ciphertext_bytes[0..12]);
    ciphertext_bytes.drain(0..12);
    // Reconstruct counter
    let mut counter: Vec<u32> = Vec::with_capacity(4);
    for i in 0..3{
        let chunks: [u8; 4] = [iv[0 + i * 4], iv[1 + i * 4], iv[2 + i * 4], iv[3 + i * 4]];
        counter.push(u32::from_le_bytes(chunks));
    }
    counter.push(0);

    // Get key from file
    let keys: Vec<u64> = read_hex_file_into_u64_byte_vec("text_files/keys.txt");
    let expanded_keys: [u64; 34] = expand_keys(&keys);

    // Split ciphertext into full blocks that can be quickly xord with encrypted counter as u64 words
    let num_full_blocks: usize = ciphertext_bytes.len() / 16;
    let ciphertext_full_blocks: Vec<u8> = (&ciphertext_bytes[0..num_full_blocks * 16]).to_vec();

    // Get remainder of the ciphertext as a u8 vector for partial xor with counter
    let ciphertext_remainder: Vec<u8> = (&ciphertext_bytes[num_full_blocks * 16..ciphertext_bytes.len()]).to_vec();
    // Function converts words into litte endian order bytes
    let ciphertext_words: Vec<u64> = bytes_to_words(ciphertext_full_blocks);
    
    // Create deciphered_text vector
    let mut deciphered_text_blocks: Vec<u64> = Vec::new();
    let mut deciphered_text_remainder: Vec<u8> = Vec::new();

    // Perform full decryption of ciphertext
    // LATER I NEED to ensure the number of ciphertext blocks does not
    // exceed 32 bits of memory
    // Turns 4 u32 chunks into 2 u64 chunks. Preserves order.
    let (concat_counter_0, concat_counter_1) = concatenate(&counter);

    // We want to save the original latter end of the counter for authentication but also
    // increment it for decryption
    let mut incrementable_counter_1: u64 = concat_counter_1;
    for i in (0..ciphertext_words.len()).step_by(2) {
        let (enc_counter_1, enc_counter_0) = encrypt(incrementable_counter_1, concat_counter_0, &expanded_keys);
        incrementable_counter_1 += 1;
        deciphered_text_blocks.push(enc_counter_0 ^ ciphertext_words[i]);
        deciphered_text_blocks.push(enc_counter_1 ^ ciphertext_words[i + 1]);
    }
    // Decrypt the remaining ciphertext bits
    let (enc_counter_1, enc_counter_0) = encrypt(incrementable_counter_1, concat_counter_0, &expanded_keys);
    let enc_counter_rem_0: [u8; 8] = enc_counter_0.to_le_bytes();
    let enc_counter_rem_1: [u8; 8] = enc_counter_1.to_le_bytes();
    for i in 0..ciphertext_remainder.len() {
        if i < 8 {
            deciphered_text_remainder.push(ciphertext_remainder[i] ^ enc_counter_rem_0[i]);
        } else{
            deciphered_text_remainder.push(ciphertext_remainder[i] ^ enc_counter_rem_1[i % 8]);
        }
    }
    // Write to deciphered.txt
    let mut deciphered_text_file_bytes: Vec<u8> = words_to_bytes(&deciphered_text_blocks);
    deciphered_text_file_bytes.extend_from_slice(&deciphered_text_remainder);
    if let Err(e) = write_deciphered_text_from_u8_byte_vec("text_files/decipheredtext.txt", deciphered_text_file_bytes) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Decrypted values written to {}", "text_files/decipheredtext.txt");
    }
}
