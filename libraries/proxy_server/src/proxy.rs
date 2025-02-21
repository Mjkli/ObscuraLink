
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use local_ip_address::list_afinet_netifas;



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// Parse Request
fn parse_request(request: &str) -> Option<String> {
   if request.starts_with("CONNECT") {
        let parts: Vec<&str> = request.split_whitespace().collect();
        if parts.len() > 1 {
            return Some(parts[1].to_string());
        }
   }
   None
}

fn handle_client(mut client_stream: TcpStream) {
    
    let mut buffer = [0; 1024];
    if let Ok(bytes_read) = client_stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        
        if let Some(destination) = parse_request(&request) {
            println!("Tunneling to: {}", destination);
            tunnel(client_stream, &destination);
        }
    
    }
}


fn tunnel(mut client_stream: TcpStream, destination: &str) {

    match TcpStream::connect(destination) {
        Ok(mut server_stream) => {
            let response = b"HTTP/1.1 200 Connection Established\r\n\r\n";
            client_stream.write_all(response).unwrap();

    
            let mut client_clone = client_stream.try_clone().unwrap();
            let mut server_clone = server_stream.try_clone().unwrap();

            // Forward data in both directions
            thread::spawn(move || std::io::copy(&mut client_clone, &mut server_stream).unwrap());
            thread::spawn(move || std::io::copy(&mut server_clone, &mut client_stream).unwrap());


        }
        Err(err) => {
            eprintln!("Failed to connect to {}: {}", destination, err);
        }

    }

}


pub fn proxy(){

    let net_ints = list_afinet_netifas().unwrap();

    for (_, ip) in net_ints.iter() {
      for stream in TcpListener::bind(format!("{}:3000", ip)).unwrap().incoming() {
      
        match stream {
            Ok(client_stream) => {
                thread::spawn(move || handle_client(client_stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e)

        }
      }
    }
}


