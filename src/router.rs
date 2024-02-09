use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::messaging::{Request, Response};

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
                                        Response::http_error("404", "Not found")
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
