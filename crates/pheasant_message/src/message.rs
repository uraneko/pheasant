use request::lex::lex;

pub mod error;
pub mod forward;
pub mod preflight;
pub mod request;
pub mod respond;

pub use error::ErrorMessage;
pub use forward::Forward;
pub use preflight::Preflight;
pub use request::Request;
pub use respond::Respond;

impl Message {
    pub fn request<R: Requester>(buf: &mut [u8], read: &mut R) -> Self {
        let tokens = lex(read, buf)?;
        if tokens.is_empty() {}

        Request::parse(tokens).into()
        // match Request::parse(tokens) {
        //     Ok(req) => req.into(),
        //     Err(err) => err.into(),
        // }
    }

    pub fn error(err: StatusError) -> Self {
        todo!()
    }

    pub fn respond(self, params: ResourceParams<'_>) -> Self {
        let Self::Request(req) = self else { panic!() };

        Respond::initialize(req, params).into()
    }

    pub fn forward(self, dest: Route) -> Self {
        let Self::Request(req) = self else { panic!() };

        Forward::initialize(req, dest).into()
    }

    pub fn preflight(self, cors: &ResourceCors) -> Self {
        let Self::Preflight(prf) = self else { panic!() };

        Preflight::initialize(req, cors).into()
    }

    pub fn is_err(&self) -> bool {
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
    Error(ErrorMessage),
}
