extern crate rand;

mod file_handling;
mod encryption;
mod handle_random;
mod utils;

use file_handling::read_file_into_byte_vec;

use encryption::encrypt;
use encryption::expand_keys;
use encryption::bytes_to_words;

use handle_random::generate_key;
use handle_random::generate_counter;

use utils::concatenate;


fn main() {
    // Get plaintext from file and convert to byte vec
    let plaintext: Vec<u8> = read_file_into_byte_vec("plaintext.txt");
    let plaintext_words: Vec<u64> = bytes_to_words(&plaintext);
    println!("{:?}", plaintext_words);
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
    let mut ciphertext: Vec<u64> = Vec::new();

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
        ciphertext.push(enc_counter_0 ^ plaintext_words[i]);
        ciphertext.push(enc_counter_1 ^ plaintext_words[i + 1]);
    }
    println!("{:?}", ciphertext);
    // Create tag with GHASH function

    // Write IV, ciphertext, tag to ciphertext.txt

    // Write key to keys.txt
}