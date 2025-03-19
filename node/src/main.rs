
mod types;

#[macro_use] extern crate rocket;

use types::ConnectionDB;
use std::str;
use std::sync::{Mutex, Arc};
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use rocket::State;
use std::net::IpAddr;



type DB = Arc<Mutex<ConnectionDB>>;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"

}


#[get("/key")]
fn get_key(db: &State<DB>, ip: IpAddr) -> Vec<u8> {
  
    let mut map = db.lock().unwrap();
    //printing for Debugging
    println!("map: {:?}", map);  
    
    
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();
    
    if map.clients.contains_key(&ip.to_string()) {
        let key = map.clients.get(&ip.to_string());
        return match key {
            Some(out) => out.public_key_to_pem().unwrap(),
            None => panic!()
        };
    } else {
        map.clients.insert(ip.to_string(), pkey.clone());
        pkey.public_key_to_pem().unwrap()
    }
    

}




#[launch]
fn rocket() -> _ {

    let db = Arc::new(Mutex::new(ConnectionDB::new()));    

    rocket::build()
        .manage(db)
        .mount("/", routes![index,get_key])
}
