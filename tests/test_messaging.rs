#[cfg(test)]
mod tests{
    use rust_webserver::{messaging::{self, Response}, utils::{self, get_static_file}};

    /// This will check wether or not it looks for an index file inside static
    #[test]
    fn response_from_html() {
        _ = messaging::Response::html_response("./index.html").unwrap();

    }

    #[test]
    fn can_send_html() {
        let file_format: &str = utils::get_file_format("index.html")
            .unwrap();
        let file = get_static_file("./static/", "index.html").unwrap();

        // TODO response should start taking &str;
        let response = Response::file_response(file_format, file.clone());

        assert_eq!(response.status, "200 OK");
        assert_eq!(response.body, file);
    }
}
