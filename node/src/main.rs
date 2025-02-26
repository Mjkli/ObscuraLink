
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use std::str;


#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"

}


#[get("/config")]
fn key() -> &'static str {
    
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();
    let out = pub_key.clone();
    str::from_utf8(out.as_slice()).unwrap()
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, key])
}
