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
pub fn write_file_from_byte_vec(path: &str, bytes :Vec<u8>) -> Result<()>{
    let mut file = File::create(path).expect("Failed to create file.");
    for byte in bytes{
        write!(file, "{:02x}", byte)?;
    }
    Ok(())
}
