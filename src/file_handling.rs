use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;


// Read file into a byte vector that can then be encrypted or decrypted
pub fn read_file_into_byte_vec(path: &str) -> Vec<u8> {
    let text: String = read_to_string(path).expect("Error in reading the file");
    let bytes: Vec<u8> = text.as_bytes().to_vec();
    return bytes

}
// Writes file from byte vector (encrypted or decrypted)
fn write_file_from_byte_vec(path: &str, bytes :Vec<u8>){
    let mut file = File::create(path).expect("Failed to create file.");
    file.write_all(&bytes).expect("Failed to write to file.");
    file.flush().expect("Failed to flush file.")
}
