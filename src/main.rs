use rust_webserver::{messaging::{Request, Response}, router::Router, server::Server};



fn handler(req: Request) -> Response {
    drop(req);
    return Response::file_response("index.html");
}

fn main() {
    let server = Server::new("127.0.0.1:8080");

    let mut router = Router::new();

    router.handle_get("/", &handler);

    server.serve(router);
}

