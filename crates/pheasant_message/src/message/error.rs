struct HttpError {
    resource: String,
    method: Method,
    proto: Protocol,
    status: u16,
}
