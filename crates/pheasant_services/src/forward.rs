use pheasant_http::{Status, server::Respond};

pub struct Forward {
    status: Status,
    location: &'static str,
}

impl Forward {
    pub fn new(location: &'static str, status: Status) -> Self {
        Self { status, location }
    }

    /// writes the forward status and location to the given server Respond instance
    pub fn write(self, resp: &mut Respond) {
        resp.status(self.status);
        resp.headers_mut()
            .extend([b"location: ", self.location.as_bytes(), b"\n"].concat());
    }
}
