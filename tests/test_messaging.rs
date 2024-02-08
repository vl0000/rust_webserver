use rust_webserver::messaging;



#[cfg(test)]
mod tests{
    use rust_webserver::messaging;

    /// This will check wether or not it looks for an index file inside static
    #[test]
    pub fn response_from_html() {
        _ = messaging::Response::html_response("./index.html").unwrap();

    }
}
