extern crate rand;

mod file_handling;
mod encryption;
mod handle_random;
mod utils;

use file_handling::read_file_into_byte_vec;
use file_handling:: write_file_from_byte_vec;

use encryption::encrypt;
use encryption::expand_keys;
use encryption::bytes_to_words;
use encryption::words_to_bytes;

use handle_random::generate_key;
use handle_random::generate_counter;

use utils::concatenate;


fn main() {
    // Get plaintext from file and convert to byte vec
    let plaintext_bytes: Vec<u8> = read_file_into_byte_vec("plaintext.txt");

    // Split text into full blocks that can be quickly xord with encrypted counter as u64 words
    let num_full_blocks: usize = plaintext_bytes.len() / 16;
    let plaintext_full_blocks: Vec<u8> = (&plaintext_bytes[0..num_full_blocks * 16]).to_vec();

    // Get remainder of the plaintext as a u8 vector for partial xor with counter
    let plaintext_remainder: Vec<u8> = (&plaintext_bytes[num_full_blocks * 16..plaintext_bytes.len()]).to_vec();
    // Function converts words into litte endian order bytes
    let plaintext_words: Vec<u64> = bytes_to_words(plaintext_full_blocks);
    // println!("{:?}", plaintext_words);

    // Get authtext from file and convert to byte vec

    // Generate random key
    let keys: Vec<u64> = generate_key();
    let expanded_keys: [u64; 34] = expand_keys(&keys);
    // Create H value by encrypting zero array
    let zero: u64 = 0;
    let h: (u64, u64) = encrypt(zero, zero, &expanded_keys);
    // Generate counter (includes random IV)
    let counter: Vec<u32> = generate_counter();
    // Create ciphertext vector
    let mut ciphertext_blocks: Vec<u64> = Vec::new();
    let mut ciphertext_remainder: Vec<u8> = Vec::new();

    // Perform full encryption of plaintext
    // LATER I NEED to ensure the number of plaintext blocks does not e
    // exceed 32 bits of memory
    // Turns 4 u32 chunks into 2 u64 chunks. Preserves order.
    let (concat_counter_0, concat_counter_1) = concatenate(counter);

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
    let mut ciphertext_file_bytes: Vec<u8> = words_to_bytes(&ciphertext_blocks);
    ciphertext_file_bytes.extend_from_slice(&ciphertext_remainder);
    if let Err(e) = write_file_from_byte_vec("ciphertext.txt", ciphertext_file_bytes) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Hex values written to {}", "ciphertext.txt");
    }
    
    
    // Write key to keys.txt
}