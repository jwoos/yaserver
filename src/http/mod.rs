pub mod request;
pub mod response;

#[derive(Debug, Copy, Clone)]
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

pub fn string_to_http_method(string: &str) -> Option<HTTPMethod> {
    return match string {
        "GET" => Some(HTTPMethod::GET),
        "HEAD" => Some(HTTPMethod::HEAD),
        "POST" => Some(HTTPMethod::POST),
        "PUT" => Some(HTTPMethod::PUT),
        "DELETE" => Some(HTTPMethod::DELETE),
        "CONNECT" => Some(HTTPMethod::CONNECT),
        "OPTIONS" => Some(HTTPMethod::OPTIONS),
        "TRACE" => Some(HTTPMethod::TRACE),
        "PATCH" => Some(HTTPMethod::PATCH),
        _ => None,
    };
}

#[derive(Debug, Copy, Clone)]
pub enum HTTPVersion {
    HTTP_1_1,
    HTTP_2_0,
}

pub fn string_tp_http_version(string: &str) -> Option<HTTPVersion> {
    return match string {
        "HTTP/1.1" => Some(HTTPVersion::HTTP_1_1),
        "HTTP/2.0" => Some(HTTPVersion::HTTP_2_0),
        _ => None,
    };
}

pub enum HTTPCode {
    OK,
}
