
mod types;

#[macro_use] extern crate rocket;

use types::{ ConnectionDB, ConnectionInfo };
use std::sync::{Mutex, Arc};
use rocket::State;
use std::net::IpAddr;
use openssl::encrypt::Decrypter;

use reqwest;



type DB = Arc<Mutex<ConnectionDB>>;



#[get("/new")]
fn get_key(db: &State<DB>, ip: IpAddr) -> Vec<u8> {
  
    let mut map = db.lock().unwrap();
    
    
    if map.clients.contains_key(&ip.to_string()) {
        let key = map.clients.get(&ip.to_string());
        return match key {
            Some(out) => out.get_public(),
            None => panic!()
        };
    } else {
        
        map.clients.insert(ip.to_string(), ConnectionInfo::new());
        map.clients.get(&ip.to_string()).unwrap().get_public()
    }
    

}



#[get("/parse/<given>")]
fn parse(db: &State<DB>, given: String, ip: IpAddr) -> &'static str {
    let map = db.lock().unwrap();
    let key = &map.clients.get(&ip.to_string()).unwrap().server_key;
    let mut decrypter = Decrypter::new(&key).unwrap();


    ""
}








#[launch]
fn rocket() -> _ {
    //Setup Node and register to manager ( hard coded IP for now )
    
    let db = Arc::new(Mutex::new(ConnectionDB::new()));    


    let _ = match reqwest::blocking::get("http://192.168.0.125:8001/register") {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    };




    rocket::build()
        .manage(db)
        .mount("/", routes![get_key])
}
