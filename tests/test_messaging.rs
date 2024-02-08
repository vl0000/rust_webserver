use rust_webserver::messaging;



#[cfg(test)]
mod tests{
    use rust_webserver::messaging;

    /// This will check wether or not it can find a html file.
    /// In the future it should be changed to look for it inside a folder named ../static
    #[test]
    pub fn response_from_html() {
        _ = messaging::Response::html_response("./index.html").unwrap();

    }
}
