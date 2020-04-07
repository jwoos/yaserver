pub struct Response {
    data: std::vec::Vec<String>,
}

impl Response {
    pub fn new(data: std::vec::Vec<String>) -> Response {
        return Response { data };
    }
}
