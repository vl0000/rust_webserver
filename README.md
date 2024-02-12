# Introduction
This is a small http 1.1 webserver i made iterating a little on what i learned from chapter 20 of the rust book. It only supports GET requests for now, and is single-threaded. It's also not fully RFC 9112 compliant.

## Usage

Firstly, we have three key components, the router, a handler(The function that processes any requests), and the server struct.
The first thing we will need is to setup the server. The only thing necessary for now is to give it an IP address and port to bind to.
then, we create a handler, a router, and give the handler to the router as below:

```Rust
use rust_webserver::{messaging::{Request, Response}, router::Router, server::Server};



fn handler(req: Request) -> Response {
    return Response::file_response("index.html");
}

fn main() {
    let server = Server::new("127.0.0.1:8080");

    let mut router = Router::new();

    // This method will bind the route to the handler, but only for get requests.
    router.handle_get("/", &handler);

    // This will run the server
    server.serve(router);
}
```