pub fn get_file_format(file_name: &str) -> Result<&str, &str>{
    match file_name.split(".").last() {
        Some(format) if format != file_name => return Ok(format),
        _ => Err("Invalid file format")
    }
}