use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


pub fn proxy(ip: &str, destination: &str){
    
    for stream in TcpListener::bind(format!("{}:3000", ip)).unwrap().incoming() { 
        match stream {
            Ok(client_stream) => {
                let dest = destination.to_string();
                thread::spawn(move || handle_client(client_stream, dest));
            }
            Err(e) => eprintln!("Connection failed: {}", e)

        }
    }
}


fn handle_client(mut client_stream: TcpStream, destination: String) {
    println!("New Connection received"); 
    let mut buffer = [0; 1024];
    

    if let Ok(bytes_read) = client_stream.read(&mut buffer) {
        println!("Request received");
        let mut request = String::from_utf8_lossy(&buffer[..bytes_read]).as_ref().to_owned();
        if destination == "" {
            if let Some(destination) = parse_request(&request) {
                println!("Tunneling to: {}", destination);
                tunnel(client_stream, destination.as_str(), &mut request);
            }
        } else {
            println!("Tunneling to: {}:3000", destination);
            forward_tunnel(client_stream, format!("{}:3000", &destination).as_str(), &mut request);
        }
    
    }
}


fn parse_request(request: &str) -> Option<String> {
   println!("Request: {:?}", request);
   if request.starts_with("CONNECT") {
       println!("CONNECT Request found!"); 
       let parts: Vec<&str> = request.split_whitespace().collect();
        if parts.len() > 1 {
            println!("Sending to: {}", parts[1].to_string());
            return Some(parts[1].to_string());
        }
   }
   None
}

fn tunnel(mut client_stream: TcpStream, destination: &str, request: &mut str) {

    println!("Connecting to {}", destination);
    println!("Request: {:?}", request); 
    match TcpStream::connect(destination) {
        Ok(mut dest_stream) => {
            
            println!("Connection established, forwarding data..");

            let response = b"HTTP/1.1 200 Connection Established\r\n\r\n";
            if let Err(e) = client_stream.write_all(response) {
                eprintln!("Failed to send response: {}", e);
                return
            }
            

            let mut client_clone = match client_stream.try_clone() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to clone client stream: {}", e);
                    return
                }
            };
            let mut dest_clone = match dest_stream.try_clone() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to clone server stream: {}", e);
                    return
                }
            };

            println!("Server data forwarding...");

            // Forward data in both directions
            let client_to_server = thread::spawn(move || { 
                println!("client -> Server forwarding started.");
                if let Err(e) = std::io::copy(&mut client_clone, &mut dest_stream) {
                    eprintln!("Client -> Server error: {}", e);
                }
                println!("Client-> Server forwarding stopped");
            });
            let server_to_client = thread::spawn(move || { 
                println!("Server -> client forwarding started");
                if let Err(e) = std::io::copy(&mut dest_clone, &mut client_stream) {
                    eprintln!("Server -> Client error: {}", e);
                }
                println!("Server -> client forwarding stopped");
            });

            println!("Waiting to keep connection alive");
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

        }
        Err(err) => {
            eprintln!("Failed to connect to {}: {}", destination, err);
        }

    }

}

fn forward_tunnel(mut client_stream: TcpStream, destination: &str, request: &mut str) {

    println!("Connecting to {}", destination);
    println!("Request: {:?}", request); 
    match TcpStream::connect(destination) {
        Ok(mut dest_stream) => {
            
            println!("Connection established, forwarding data..");

            // let response = b"HTTP/1.1 200 Connection Established\r\n\r\n";
            // if let Err(e) = client_stream.write_all(response) {
            //     eprintln!("Failed to send response: {}", e);
            //     return
            // }
            
            if let Err(e) = dest_stream.write_all(request.as_bytes()) {
                eprintln!("Failed to send CONNECT request: {}", e);
                return
            }

            let mut proxy_response = [0;1024];
            if let Ok(bytes_read) = dest_stream.read(&mut proxy_response) {
                if bytes_read > 0 {
                    client_stream.write_all(&proxy_response[..bytes_read]).ok();
                }
            }



            let mut client_clone = match client_stream.try_clone() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to clone client stream: {}", e);
                    return
                }
            };
            let mut dest_clone = match dest_stream.try_clone() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to clone server stream: {}", e);
                    return
                }
            };

            println!("Server data forwarding...");

            // Forward data in both directions
            let client_to_server = thread::spawn(move || { 
                println!("client -> Server forwarding started.");
                if let Err(e) = std::io::copy(&mut client_clone, &mut dest_stream) {
                    eprintln!("Client -> Server error: {}", e);
                }
                println!("Client-> Server forwarding stopped");
            });
            let server_to_client = thread::spawn(move || { 
                println!("Server -> client forwarding started");
                if let Err(e) = std::io::copy(&mut dest_clone, &mut client_stream) {
                    eprintln!("Server -> Client error: {}", e);
                }
                println!("Server -> client forwarding stopped");
            });

            println!("Waiting to keep connection alive");
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

        }
        Err(err) => {
            eprintln!("Failed to connect to {}: {}", destination, err);
        }

    }

}




