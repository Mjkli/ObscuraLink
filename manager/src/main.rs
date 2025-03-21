


#[macro_use] extern crate rocket;

use std::sync::{Mutex, Arc};
use rocket::State;
use rocket::launch;
use std::net::IpAddr;
use std::collections::HashMap;
use std::time::SystemTime;
use rand::Rng;



#[get("/register")]
fn register_node(nodes: &State<Arc<Mutex<Vec<String>>>>, ip: IpAddr) -> &'static str{
    // Hook for nodes to register themeselves


    let mut nodes_map = nodes.lock().unwrap();

    if nodes_map.contains(&ip.to_string()) {
       "registered already!" 
    } else {
        nodes_map.push(ip.to_string());
        "registered!"
    }

}




#[get("/get_node")]
fn get_node(nodes: &State<Arc<Mutex<Vec<String>>>>, ip: IpAddr) -> String {
    // Function to where clients / nodes can request a node from the list of nodes
    
    let nodes_list = nodes.lock().unwrap();
    
    if nodes_list.contains(&ip.to_string()) {

        
        let num = rand::rng().random_range(0..nodes_list.len());
        nodes_list[num].to_string()

    } else {
        
        let num = rand::rng().random_range(0..nodes_list.len());
        nodes_list[num].to_string()

    }
    
}





#[launch]
fn rocket() -> _ {
    
    //Using a new hashmap on startup until we have a dedicated database to retrive
    // let node_list = Arc::new(Mutex::new(HashMap::<String,SystemTime>::new()));
    let node_list = Arc::new(Mutex::new(Vec::<String>::new()));


    rocket::build()
       .manage(node_list)
       .mount("/", routes![register_node, get_node])
}

