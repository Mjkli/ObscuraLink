
use std::collections::HashMap;

use openssl::rsa::Rsa;
use openssl::pkey::{PKey, Private, Public};



#[derive(Debug)]
pub struct ConnectionInfo {
    pub server_key: Rsa<Private>,
    pub client_key: PKey<Public>
}


impl ConnectionInfo {

    pub fn new(client_key: PKey<Public>) -> ConnectionInfo { 
        ConnectionInfo {
            server_key: Self::get_key(),
            client_key
        }
            
    }
    
    fn get_key() -> Rsa<Private>{
        Rsa::generate(2048).unwrap()
    }
}



#[derive(Debug)]
pub struct ConnectionDB {
    clients: HashMap<String, ConnectionInfo>
}

impl ConnectionDB {
    pub fn new() -> ConnectionDB {
        ConnectionDB {
            clients: HashMap::new()
        }
    }
}
