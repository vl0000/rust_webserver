use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::{messaging::{Request, Response}, utils};

/// Stores routes by method, then route.
pub struct Router {
        pub get: HashMap<String, Box<dyn Fn(Request) -> Response >>,
}

impl Router {
    /// Adds a new handler for GET requests
    pub fn handle_get (&mut self, path: &str, handler: &'static dyn Fn(Request) -> Response) {
        self.get.insert(path.to_string(), Box::new(handler));
    }
    /// Takes a valid request and routes it to the apropriate handler
    /// If one is not found, it writes a 404 response to the stream
    pub fn consume_request(&self, stream: &mut TcpStream){
        let req = Request::from_stream(stream);
        println!("{}", &req.endpoint);
        
        if req.method == "GET" {
            let response: Response = match self.get.get(&req.endpoint) {
                Some(handler) => {
                        handler(req)
                },
                None => {
                    // let file_response handle the error by feeding an invalid format.
                    let file_format = match utils::get_file_format(&req.endpoint) {
                        Ok(format) => format,
                        _ => "unknown"
                    };
                    
                    // Checks if the endpoint refers to a file
                    // May return a 404
                    Response::file_response(file_format, &req.endpoint)
                }
        };
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
            // If the wrong method is used, the server does not respond
            //TODO respond
    }

    pub fn new() -> Router {
            Router{
                    get: HashMap::new()
            }
    }

}
