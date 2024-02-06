use std::net::{self, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn handle_connect(mut stream: &TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Ok!</h1>";

    stream.write_all(response.as_bytes()).unwrap();


    println!("{}    {}", request[0], request[1]);

}

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {

        handle_connect(&stream.unwrap());
    }

}
