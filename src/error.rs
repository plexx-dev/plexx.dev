pub fn get_error_message(code: u16) -> String {
    match code {
        300 => "Multiple Choices".to_string(),
        301 => "Moved Permanently".to_string(),
        304 => "Not Modified".to_string(),
        307 => "Temporary Redirect (since HTTP/1.1)".to_string(),
        308 => "Permanent Redirect".to_string(),

        400 => "Bad Request".to_string(),
        401 => "Unauthorized".to_string(),
        403 => "Forbidden".to_string(),
        404 => "Not found".to_string(),
        405 => "Method not allowed".to_string(),
        408 => "Request Timeout".to_string(),
        418 => "I'm a Teapot".to_string(),

        500 => "Internal Server Error".to_string(),
        502 => "Bad Gateway".to_string(),
        503 => "Service Unavailable".to_string(),

        _ => "WIP ERROR CODE".to_string(),
    }
}