
use openssl::symm::{encrypt, Cipher};
use openssl::rand::rand_bytes;

pub mod encrypt_text;


// pub fn get_key_iv() -> (Vec<u8>, Vec<u8>){

//     let cipher = Cipher::aes_256_cbc();
//     let key_length = cipher.key_len();
//     let iv_length = cipher.iv_len().unwrap();

//     let mut key = vec![0u8; key_length];
//     rand_bytes(&mut key).expect("Failed to gen key");


//     let mut iv = vec![0u8; iv_length];
//     rand_bytes(&mut iv).expect("Failed to generate IV");

//     return (key, iv);
// }
