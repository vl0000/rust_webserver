


#[cfg(test)]
mod tests {
    use rust_webserver::{messaging::{Request, Response}, router::Router};

    fn handler(req: Request) -> Response {

        drop(req);
        
        Response::new(
            "HTTP/1.1".to_string(),
            "200 OK".to_string(),
            "".to_string(),
            vec!["Content-type: text/html".to_string()]
        )        
    }

    #[test]
    fn can_create_router() {
        _ = Router::new();
    }

    #[test]
    fn accepts_handler() {
        let mut my_router = Router::new();

        my_router.handle_get("/", &handler);
    }

}