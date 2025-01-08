
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
mod encrypt;

fn main() {
    let (key, iv) = encrypt::get_key_iv();
    let test_data = "THIS IS A TEST STRING!";
    let encrypted_data = encrypt::encrypt_text(test_data, &key, &iv);
    let b64_data = STANDARD_NO_PAD.encode(encrypted_data);
    println!("base64 encoded: {}", b64_data);
}
