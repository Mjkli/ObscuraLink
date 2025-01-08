


use encrypt::get_key_iv;


fn main() {
    // Get iv / key for encryption
    // Create capture process and pass in the iv / key for decryption
    // Create sending process and pass in the iv / key for encryption
    // monitor output of both to verify it is working.
    let (key, iv) = get_key_iv();
    
    println!("key: {:?}", key);
    println!("iv: {:?}", iv);




}
