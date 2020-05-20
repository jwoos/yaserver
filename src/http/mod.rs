pub mod request;
pub mod response;

pub enum HTTPMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

pub fn string_to_http_method(string: &str) -> Result<HTTPMethod, ()> {
    return match string {
        "GET" => Ok(HTTPMethod::GET),
        "HEAD" => Ok(HTTPMethod::HEAD),
        "POST" => Ok(HTTPMethod::POST),
        "PUT" => Ok(HTTPMethod::PUT),
        "DELETE" => Ok(HTTPMethod::DELETE),
        "CONNECT" => Ok(HTTPMethod::CONNECT),
        "OPTIONS" => Ok(HTTPMethod::OPTIONS),
        "TRACE" => Ok(HTTPMethod::TRACE),
        "PATCH" => Ok(HTTPMethod::PATCH),
        _ => Err(()),
    };
}

pub enum HTTPCode {
    OK,
}
