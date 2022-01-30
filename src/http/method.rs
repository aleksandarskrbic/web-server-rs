pub enum Method {
    GET(String),
    POST,
    PUT,
    DELETE(u64),
    HEAD,
}
