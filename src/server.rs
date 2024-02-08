use crate::router::Router;
use std::net::{self, TcpListener};
pub struct Server {
    /// The address containing a port. Ex: 127.0.0.1:8080
    pub address: String,
}
impl Server {
    pub fn serve(&self, router: Router) {
        let listener:TcpListener = match net::TcpListener::bind(&self.address) {
            Ok(listener) => listener,
            Err(e) => panic!("{}", e)
        };
        // TODO make it multithreaded
        for stream in listener.incoming() {

            router.consume_request(&mut stream.unwrap());

        }
    }

    pub fn new(address: &str) -> Server{
        Server { address: address.to_string() }
    }

}