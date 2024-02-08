use std::net::{self, TcpStream};
use std::io::Write;
use rust_webserver::messaging::{Request, Response};

fn handle_connect(mut stream: &TcpStream) {

    let resp = Response::html_response("./index.html").unwrap();

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
