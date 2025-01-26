extern crate rand;

mod gcm;
mod file_handling;
mod handle_random;
mod utils;
mod speck;

use gcm::decrypt_to_file;

use crate::gcm::encrypt_to_file;

fn main(){
    encrypt_to_file();
    decrypt_to_file();
}