pub mod error;
pub mod forward;
pub mod preflight;
pub mod request;
pub mod respond;

impl Message {
    fn request<R: Requester>(read: &mut R) -> Self {
        Self::Request(read.read_req())
    }

    fn respond(self, params: &ResourceParams) -> Self {
        let Self::Request(req) = self else { panic!() };

        let resp_init = Respond::initialize(req, params);
        Self::Respond(resp_init)
    }

    fn is_err(&self) -> bool {
        match self {
            Self::ErrorOut { .. } => true,
            _ => false,
        }
    }
}

pub enum Message {
    Request(Request),
    Respond(Respond),
    Forward(Forward),
    Preflight(Preflight),
    Error(HttpError),
}
