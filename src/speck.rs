fn round(mut x: u64, mut y: u64, k: u64) -> (u64, u64){
    const ALPHA: u32 = 8;
    const BETA: u32 = 3;
    let x_right_rotation: u64 = x.rotate_right(ALPHA).wrapping_add(y);
    x = x_right_rotation ^ k;
    y = y.rotate_left(BETA) ^ x_right_rotation ^ k;
    return (x, y)
}
pub fn expand_keys(key: &Vec<u64>) -> [u64; 34]{
    let mut l_arr: [u64; 3] = [0; 3];
    let mut k_arr: [u64; 34] = [0; 34];
    k_arr[0] = key[0];
    l_arr[0] = key[1];
    l_arr[1] = key[2];
    l_arr[2] = key[3];
    for i in 0..33 {
        (l_arr[i % 3], k_arr[i + 1]) = round(l_arr[i % 3], k_arr[i], i as u64)
    }
    return k_arr
}
pub fn words_to_bytes(words: &Vec<u64>) -> Vec<u8>{
    let mut byte_vec: Vec<u8> = Vec::new();
    for &word in words{
        let eight_byte_piece = word.to_le_bytes();
        byte_vec.extend_from_slice(&eight_byte_piece);
    }
    return byte_vec
}
pub fn bytes_to_words(bytes: Vec<u8>) -> Vec<u64>{
    bytes.chunks(8).map(|chunk| {
        let mut chunk_byte: [u8; 8] = [0; 8];
        chunk_byte.copy_from_slice(chunk);
        u64::from_le_bytes(chunk_byte)
    }).collect()
}
pub fn encrypt(mut x: u64, mut y: u64, expanded_keys: &[u64; 34]) -> (u64, u64){
    for i in 0..34 {
        (x, y) = round(x, y, expanded_keys[i])
    }
    return (x, y)
}
// fn main(){
//     let plaintext_bytes: Vec<u8> = vec![0x70, 0x6f, 0x6f, 0x6e, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x6f, 0x73, 0x65];
//     let key_bytes: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
//                                   0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f];
//     let keys: Vec<u64> = bytes_to_words(&key_bytes);
//     let expanded_keys: [u64; 34] = expand_keys(&keys);

//     let words: Vec<u64> = bytes_to_words(&plaintext_bytes);
//     let words_length: usize = words.len();
    
//     let mut encrypted_words: Vec<u64> = Vec::new();
//     for i in (0..words_length).step_by(2) {
//         let encrypted_0: u64;
//         let encrypted_1: u64;
//         (encrypted_1, encrypted_0) = encrypt(words[i+1], words[i], &expanded_keys);
//         encrypted_words.push(encrypted_0);
//         encrypted_words.push(encrypted_1);
//     }
    
//     let ciphertext: Vec<u64> = vec![encrypted_words[0], encrypted_words[1]];
//     let cipher_bytes: Vec<u8> = words_to_bytes(&ciphertext);
    
//     for byte in cipher_bytes{
//         println!("{:x}", byte);
//     }
// }