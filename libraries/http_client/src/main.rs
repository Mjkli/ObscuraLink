
use http_body_util::Empty;
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    
    // Setup URL
    let url = "http://www.google.com".parse::<hyper::Uri>()?;

    //Get Host and port
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    // Open a TCP connection to remote host
    let stream = TcpStream::connect(address).await?;

    // Use an adapter to access something implementing 
    let io = TokioIo::new(stream);

    //Create the Hyper client
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }

    });


    let authority = url.authority().unwrap().clone();

    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;
    
    let mut res = sender.send_request(req).await?;

    println!("Response: {:?}", res);





    Ok(())
}
