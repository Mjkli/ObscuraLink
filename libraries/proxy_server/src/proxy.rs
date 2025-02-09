
use http_body_util::{Full, Empty};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use std::convert::Infallible;
use tokio::net::{TcpListener, TcpStream};
use hyper_util::rt::TokioIo;
use http_body_util::BodyExt;

use std::net::SocketAddr;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}


async fn forward(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>,Infallible> {
    println!("Found request");
    _ = service().await; 
    Ok(Response::new(Full::new(Bytes::from("Hello World"))))
}


async fn service() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://www.google.com".parse::<hyper::Uri>()?;

    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);

    let addr = format!("{}:{}",host,port);

    let stream = TcpStream::connect(addr).await?; 
    let io = TokioIo::new(stream);



    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;


    //spawn a task to poll the connection
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection Failed: {:?}", err);
        }
    });


    let authority = url.authority().unwrap().clone();

    let path = url.path();
    let req = Request::builder()
                .uri(path)
                .header(hyper::header::HOST, authority.as_str())
                .body(Empty::<Bytes>::new())?;

    let mut res = sender.send_request(req).await?;

    
    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    

    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            // io::stdout().write_all(chunk).await?;
            print_type_of(chunk);
        }
    }
    Ok(())
}

pub async fn proxy() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    
    let listener = TcpListener::bind(addr).await?;
    
    // proxy service to catch reuqests
    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        //Spawn a tokio task to server multiple connections
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(forward))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });

    }

}



