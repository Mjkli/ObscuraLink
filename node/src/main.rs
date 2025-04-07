
mod types;

extern crate proxy_server;


#[macro_use] extern crate rocket;

use types::{ ConnectionDB, ConnectionInfo };
use std::sync::{Mutex, Arc};
use rocket::State;
use rocket::serde::json::Json;
use std::net::IpAddr;
use reqwest;



type DB = Arc<Mutex<ConnectionDB>>;



// Debating if I should combine the new / setup_proxy handle
// Main argument against is that the new handle returns a list of public keys of nodes
// and the setup_proxy will return saying the nodes are ready. 
// Although if I am going to ask to create a chain might as well create the proxy and return the
// public key and proxy settings.



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




#[get("/set_chain/<number>")]
fn set_chain(db: &State<DB>, ip: IpAddr, number: i32) {
    // if number > 0 decrement by 1 and call set_chain on another node
    let map = db.lock().unwrap();
   


    // If number is greater then 0, then ask the manager for another node
    // if we are at 0 then assume we are the last node. (setup proxy and return public key)
    if number > 0 {
      
        // Get Node from node manager
        let next_node = match reqwest::blocking::get("http://192.168.0.125:8001/") {
            Ok(node) => node.text().unwrap(),
            Err(err) => panic!("Error: {}", err)
        };
        
        // Setup Next Node Proxy

        let next_node_pub = match reqwest::blocking::get(format!("http://{}:8000/set_chain/{}",next_node, number - 1)) {
            Ok(node) => node.text().unwrap(),
            Err(err) => panic!("Error: {}", err)
        };


        // Setup Proxy for client
        
                  
        proxy_server::proxy::proxy("192.168.0.125", "", 8002);

        // node_list.push(next_node_pub);
        
    }

    // let key = get_key(db, ip);
    // node_list.push(key);


    // Json(node_list) 

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
        .mount("/", routes![set_chain])
}
