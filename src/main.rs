use std::net::{self, TcpStream};
use std::io::Write;
use rust_webserver::messaging::{Request, Response};

fn handle_connect(mut stream: &TcpStream) {

    //let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Ok!</h1>";
    let resp = Response::new("HTTP/1.1".to_string(), "200 Ok".to_string(), "<h1>Ok</h1>".to_string());
    
    stream.write_all(resp.to_string().as_bytes()).unwrap();

    let request: Request = Request::from_stream(&mut stream);

    println!("{} {} {}", request.method, request.endpoint, request.version)


}

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {

        handle_connect(&stream.unwrap());
    }

}
