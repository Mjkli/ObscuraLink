
mod types;


use types::ConnectionDB;
use std::str;
use std::sync::{Mutex, Arc};
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use rocket::State;




#[macro_use] extern crate rocket;


type DB = Arc<Mutex<ConnectionDB>>;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"

}


#[get("/key")]
fn get_key<'r>(db: &State<DB>) -> Vec<u8> {
   

    let map = db.lock().unwrap();
    println!("map: {:?}", map);  



    
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();
    pkey.public_key_to_pem().unwrap()

}




#[launch]
fn rocket() -> _ {

    let db = Arc::new(Mutex::new(ConnectionDB::new()));    

    rocket::build()
        .manage(db)
        .mount("/", routes![index,get_key])
}
