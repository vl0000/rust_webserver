use std::{io::{self, BufRead, BufReader, Error}, net::TcpStream, vec};
use std::fs;

use crate::utils;
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

/// Response builder
pub struct Response {
    pub version: String,
    pub status: String,
    pub body: String,
    pub headers: Vec<String>
}


impl Response {
    /// Returns a new response
    pub fn new(version: String, status: String, body: String, headers: Vec<String>) -> Response {
        Response {version: version, status: status, body: body, headers: headers }
    }

    pub fn to_string(&self) -> String {

        let headers = self.headers.join("\r\n");

        let response:String = format!("{} {}\r\n{}\r\n{}", self.version, self.status, headers, self.body);

        return response;
    }


    /// Creates a response, with a HTML document as a body.
    /// Made redundant by file_response.
    #[deprecated]
    pub fn html_response(path: &str) -> Result<Response, Error>{
        let document: String = get_html_document(path)?;

        let response: Response = Response {
            version: "HTTP/1.1".to_string(),
            status: "200 Ok".to_string(),
            body: document,
            headers: vec!["".to_string()]
        };
        
        Ok(response)
    }

    pub fn http_error(status_code: &str, message: &str) -> Response {
        Response::new(
                "HTTP/1.1".to_string(),
                status_code.to_string(),
                format!("<h1>{}</h1>", message),
                vec!["Content-Type: text/html".to_string()]
        )
    }

    /// Builds a response based on a file extension, JS, HTML, or CSS;
    /// In case of failure, it returns a response containing a HTTP error
    pub fn file_response(file_format: &str, file_name: &str) -> Response {

        let file: String = match utils::get_static_file("./static/", file_name) {
            Ok(file) => file,
            Err(_) => return Response::http_error("404", "Not Found")
        };

        // Javascript requires a content-type header
        //TODO implement it
        let body: String = match file_format {
            "html" => file,
            "css" => file,
            "js" => file,
            _ => return Response::http_error("404", "Not Found")
        };

        let headers: Vec<String> = match file_format {
            "js" => vec!["Content-Type: text/javascript".to_string()],
            _ => vec!["".to_string()]
        };

        Response::new(
            "HTTP/1.1".to_string(),
            "200 OK".to_string(),
            body,
            headers
        )
    }

    pub fn add_header(&mut self, header: &str) {
        self.headers.push(header.to_string());
    }
}


//TODO add a config that changes the base path
/// WIP
/// Takes a path for a html document.
fn get_html_document(path: &str) -> Result <String, io::Error> {
    let base: String = String::from("./static/");

    let file = match fs::read(base + path) {
        Ok(file) => {
            String::from_utf8_lossy(&file)
                .to_string()
        },
        Err(e) => return Err(e)
    };
    
    Ok(file)
}