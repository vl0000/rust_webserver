use std::{collections::HashMap, fs, io::Write, net::TcpStream};

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
                                None => Response::new(
                                        "HTTP/1.1".to_string(),
                                        "404".to_string(),
                                        "<h1>Route not found</h1>".to_string()
                                )
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
pub fn get_static_file(path: &str, resource: &str) -> Result<String, std::io::Error> {
    // TODO return a 404 if not found
    let file_name: &str = resource.strip_prefix("/").unwrap();

    let file = match fs::read(path.to_string() + file_name) {

        Ok(f) => {
            String::from_utf8_lossy(&f)
            .to_string()
        }
        Err(e) => {
            return Err(e)
        }

    };
    return Ok(file);
 
}