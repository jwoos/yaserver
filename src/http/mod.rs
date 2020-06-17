pub mod request;
pub mod response;

use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap(
    {$($key:expr => $value:expr),+} => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    }
);

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

pub fn string_to_http_version(string: &str) -> Option<HTTPVersion> {
    return match string {
        "HTTP/1.1" => Some(HTTPVersion::HTTP_1_1),
        "HTTP/2.0" => Some(HTTPVersion::HTTP_2_0),
        _ => None,
    };
}

pub fn http_version_to_string(version_enum: HTTPVersion) -> Option<&'static str> {
    return match version_enum {
        HTTPVersion::HTTP_1_1 => Some("HTTP/1.1"),
        HTTPVersion::HTTP_2_0 => Some("HTTP/2.0"),
        _ => None,
    };
}

// TODO change to hash map
pub fn get_code_message<'a>(code: &'a u16) -> Option<&'static str> {
    return match code {
        100 => Some("Continue"),
        101 => Some("Switching Protocols"),
        103 => Some("Early Hints"),
        200 => Some("Okay"),
        201 => Some("Created"),
        202 => Some("Accepted"),
        203 => Some("Non-Authoritative Information"),
        204 => Some("No Content"),
        205 => Some("Reset Content"),
        206 => Some("Partial Content"),
        300 => Some("Multiple Choices"),
        301 => Some("Moved Permanently"),
        302 => Some("Found"),
        303 => Some("See Other"),
        304 => Some("Not Modified"),
        307 => Some("Temporary Redirect"),
        308 => Some("Permanent Redirect"),
        400 => Some("Bad Request"),
        401 => Some("Unauthorized"),
        402 => Some("Payment Required"),
        403 => Some("Forbidden"),
        404 => Some("Not Found"),
        405 => Some("Method Not Allowed"),
        406 => Some("Not Acceptable"),
        407 => Some("Proxy Authentication Required"),
        408 => Some("Request Timeout"),
        409 => Some("Conflict"),
        410 => Some("Gone"),
        411 => Some("Length Required"),
        412 => Some("Precondition Failed"),
        413 => Some("Payload Too Large"),
        414 => Some("URI Too Long"),
        415 => Some("Unsupported Media Type"),
        416 => Some("Range Not Satisfiable"),
        417 => Some("Expectation Failed"),
        418 => Some("I'm a teapot"),
        422 => Some("Unprocessable Entity"),
        425 => Some("Too Early"),
        426 => Some("Upgrade Required"),
        428 => Some("Precondition Required"),
        429 => Some("Too Many Requests"),
        431 => Some("Request Header Fields Too Large"),
        451 => Some("Unavailable For Legal Reasons"),
        500 => Some("Internal Server Error"),
        501 => Some("Not Implemented"),
        502 => Some("Bad Gateway"),
        503 => Some("Service Unavailable"),
        504 => Some("Gateway Timeout"),
        505 => Some("HTTP Version Not Supported"),
        506 => Some("Variant Also Negotiates"),
        507 => Some("Insufficient Storage"),
        508 => Some("Loop Detected"),
        510 => Some("Not Extended"),
        511 => Some("Network Authentication Required"),
        _ => None,
    };
}
