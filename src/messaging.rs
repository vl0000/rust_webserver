use std::{io::{BufRead, BufReader}, net::TcpStream};

/// Request serialiser to make it easier to work with them
pub struct Request {
        pub method: String,
        pub endpoint: String,
        pub version: String,
}

/// Serialises a request from a TCPStream
impl Request {

    /// Takes a stream and returns a Request struct
    pub fn from_stream(mut stream: &TcpStream) -> Request {
        let buffer_reader: BufReader<&mut &TcpStream> = BufReader::new(&mut stream);
        let lines: Vec<String> = buffer_reader
        // Returns an iterator to go over each line of the buffer
        .lines()
        // Uses the iterator to apply a a closure over every line.
        // Unwrapping the Result type assumes there will be no error
        // I dont see a reason for .lines() to return a bad iterator so, i doubt that..
        // It will ever return an error
        .map(|result| result.unwrap())
        // I found out that if you do not check whether or not the lines are empty...
        // It seems to go on forever without actually returning a value.
        .take_while(|line| !line.is_empty())
        .collect();

        // Stores an iterator for the request line(The first line)
        let mut request_line = lines[0]
        .split_whitespace()
        .map(|piece| piece.to_string());

        let request = Request {
            method: request_line.next().unwrap(),
            endpoint: request_line.next().unwrap(),
            version: request_line.next().unwrap(),
        };
        
        return request
    }

}

pub struct Response {
    pub version: String,
    pub status: String,
    pub body: String,
}


impl Response {
    /// Returns a new response
    pub fn new(version: String, status: String, body: String) -> Response {
        Response {version: version, status: status, body: body }
    }

    pub fn to_string(&self) -> String {
        let response = format!("{} {}\r\n\r\n{}", self.version, self.status, self.body);

        return response;
    }
}