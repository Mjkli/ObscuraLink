
use std::collections::HashMap;

use openssl::rsa::Rsa;
use openssl::pkey::{PKey, Private, Public};

use rocket::serde::{Serialize};



#[derive(Debug)]
pub struct ConnectionInfo {
    pub server_key: PKey<Private>,
}


impl ConnectionInfo {

    pub fn new() -> ConnectionInfo { 
        ConnectionInfo {
            server_key: Self::new_key(),
        }
            
    }
    
    fn new_key() -> PKey<Private>{
        let rsa = Rsa::generate(2048).unwrap();
        PKey::from_rsa(rsa).unwrap()
    }

    pub fn get_public(&self) -> Vec<u8> {
        self.server_key.public_key_to_pem().unwrap() 
    }


}



#[derive(Debug)]
pub struct ConnectionDB {
    pub clients: HashMap<String, ConnectionInfo>
}

impl ConnectionDB {
    pub fn new() -> ConnectionDB {
        ConnectionDB {
            clients: HashMap::new()
        }
    }
}




