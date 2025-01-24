extern crate rand;

use rand::rngs::OsRng;
use rand::RngCore;

pub fn generate_key() -> Vec<u64>{
    let mut rng: OsRng = OsRng;
    let mut keys: Vec<u64> = Vec::new();
    for _ in 0..4 {
        let mut random_eight_bytes: [u8; 8] = [0; 8];
        rng.fill_bytes(&mut random_eight_bytes);
        let random_word: u64 = u64::from_le_bytes(random_eight_bytes);
        keys.push(random_word);
    }
    return keys
}
pub fn generate_counter() -> Vec<u32> {
    let mut rng: OsRng = OsRng;
    let mut counter: Vec<u32> = Vec::new();
    // The leftmost 96 bits will be the iv, the remaining 32 bits
    // is sepcific to the counter which will be initialized as 0. 
    for _ in 0..3 {
        let mut random_four_bytes: [u8; 4] = [0; 4];
        rng.fill_bytes(&mut random_four_bytes);
        let random_section: u32 = u32::from_le_bytes(random_four_bytes);
        counter.push(random_section);
    }
    counter.push(0);
    return counter
}