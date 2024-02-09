use std::fs;

pub fn get_file_format(file_name: &str) -> Result<&str, &str>{
    match file_name.split(".").last() {
        Some(format) if format != file_name => return Ok(format),
        _ => Err("Invalid file format")
    }
}

pub fn get_static_file(path: &str, resource: &str) -> Result<String, std::io::Error> {
    // TODO return a 404 if not found
    let file_name: &str = resource.strip_prefix("/").unwrap();

    let file = match fs::read(path.to_string() + file_name) {

        Ok(f) => {
            String::from_utf8_lossy(&f)
            .to_string()
        }
        Err(e) => {
            return Err(e)
        }

    };
    return Ok(file);
 
}