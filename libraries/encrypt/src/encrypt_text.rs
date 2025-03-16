use openssl::symm::{encrypt, Cipher};
use openssl::rand::rand_bytes;
use openssl::rsa::Rsa;
use openssl::pkey::Private;




pub fn get_key_iv() -> (Vec<u8>, Vec<u8>){

    let cipher = Cipher::aes_256_cbc();
    let key_length = cipher.key_len();
    let iv_length = cipher.iv_len().unwrap();

    let mut key = vec![0u8; key_length];
    rand_bytes(&mut key).expect("Failed to gen key");


    let mut iv = vec![0u8; iv_length];
    rand_bytes(&mut iv).expect("Failed to generate IV");

    return (key, iv);
}

pub fn encrypt_text(data: &str, key: &[u8], iv: &[u8]) -> Vec<u8>{
    let cipher = Cipher::aes_256_cbc(); 
    let ciphertext = encrypt(cipher, key, Some(iv), data.as_bytes());
    return ciphertext.unwrap();
}

pub fn get_async_key() -> Rsa<Private> {
    Rsa::generate(2048).unwrap()
}

