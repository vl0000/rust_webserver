#[cfg(test)]
mod tests {
    use rust_webserver::utils;
    use rust_webserver::utils::get_static_file;

    #[test]
    fn valid_file_format() {
        match utils::get_file_format("index.html") {
            Ok(format) => assert_eq!(format, "html"),
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn finds_static_file() {
        let file = get_static_file("./static/", "/style.css");
        
        match file {
            Ok(_) => assert!(true, "File found"),
            Err(_) => assert!(false, "File not found")
        }
    }
    #[test]
    fn rejects_invalid_format() {
        match utils::get_file_format("not a file") {
            Ok(format) => assert!(false, "No file format should have been detected: {}", format),
            Err(_) => assert!(true, "OK")
        }
    }
}