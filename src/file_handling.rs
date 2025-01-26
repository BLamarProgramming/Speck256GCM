use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::io::Result;


// Read file into a byte vector that can then be encrypted or decrypted
pub fn read_file_into_byte_vec(path: &str) -> Vec<u8> {
    let text: String = read_to_string(path).expect("Error in reading the file");
    let bytes: Vec<u8> = text.as_bytes().to_vec();
    return bytes

}
// Writes file from byte vector (encrypted or decrypted)
pub fn write_file_from_u8_byte_vec(path: &str, bytes :Vec<u8>) -> Result<()>{
    let mut file = File::create(path).expect("Failed to create file.");
    for byte in bytes{
        write!(file, "{:02x}", byte)?;
    }
    Ok(())
}pub fn write_file_from_u64_byte_vec(path: &str, sections: Vec<u64>) -> Result<()>{
    let mut file = File::create(path).expect("Failed to create file.");
    for section in sections{
        write!(file, "{:016x}", section)?;
    }
    Ok(())
}
pub fn write_deciphered_text_from_u8_byte_vec(path: &str, bytes :Vec<u8>) -> Result<()>{
    let mut file = File::create(path).expect("Failed to create file.");
    file.write_all(&bytes)?;
    Ok(())
}
pub fn iv_to_bytes(counter: &Vec<u32>) -> Vec<u8>{
    let mut iv: Vec<u8> = Vec::with_capacity(3); 
    for i in 0..(counter.len() - 1) {
        iv.extend_from_slice(&counter[i].to_le_bytes());
    }
    return iv
}
pub fn read_hex_file_into_u8_byte_vec(path: &str) -> Vec<u8>{
    let text: String = read_to_string(path).expect("Error in reading the file");
    let mut full_cipher_text: Vec<u8> = Vec::with_capacity(text.len() / 2);
    for i in (0..text.len() - 1).step_by(2){
        let byte: &str = &text[i..i+2];
        full_cipher_text.push(u8::from_str_radix(byte, 16).expect("Invalid hex"));
    }
    return full_cipher_text
}
pub fn read_hex_file_into_u64_byte_vec(path: &str) -> Vec<u64> {
    let hex_u8_vec: Vec<u8> = read_hex_file_into_u8_byte_vec(path);
    let mut hex_u64_vec: Vec<u64> = Vec::with_capacity(4);
    for i in 0..4{
        let mut key_word: u64 = 0;
        for j in 0..8{
            if key_word != 0 {
                key_word = key_word << 8;
            }
            key_word = key_word + hex_u8_vec[i * 8 + j] as u64;
        }
        hex_u64_vec.push(key_word);
    }
    return hex_u64_vec
}
