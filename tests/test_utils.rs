#[cfg(test)]
mod tests {
    use rust_webserver::utils;

    #[test]
    fn valid_file_format() {
        match utils::get_file_format("index.html") {
            Ok(format) => assert_eq!(format, "html"),
            Err(e) => assert!(false, "{}", e)
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