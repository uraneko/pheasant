use crate::{Failure, Request, Service};
use alloc::{format, vec::Vec};
use pheasant_core::{ErrorStatus, Method, ResponseStatus, Status, Successful};

pub enum FindService<'a> {
    Redirect {
        status: Status,
        route: &'a str,
    },
    Success {
        service: &'a Service,
        status: Status,
    },
    Error {
        failure: Result<&'a Failure, Vec<u8>>,
        status: ErrorStatus,
    },
}

impl<'a> FindService<'a> {
    pub fn success(service: &'a Service, method: Method) -> Self {
        Self::Success {
            service,
            status: match method {
                Method::Options => Status::Successful(Successful::NoContent),
                Method::Get => Status::Successful(Successful::OK),
                Method::Post => Status::Successful(Successful::Created),
                Method::Head => Status::Successful(Successful::OK),
                // TODO put can have 1 of 3 statues
                // - 204 no content | 200 ok --> pre-existing resource was modified successfuly
                // - 201 created -> new resource created
                Method::Put => Status::Successful(Successful::Created),
                // TODO patch can return any of the 2xx series of statues
                Method::Patch => Status::Successful(Successful::NoContent),
                // TODO delete can take one of <accepted | no content | ok>
                // depending on how the server handled the request
                Method::Delete => Status::Successful(Successful::Accepted),
                // NOTE I dont plans to support Trace for the time being
                Method::Trace => Status::Successful(Successful::OK),
                // NOTE this method is for proxy servers, which this framework doesnt support
                // ie, unsupported
                // can return any successful status; 2xx
                Method::Connect => Status::Successful(Successful::OK),
            },
        }
    }

    // error fallback when no failure service is found
    pub fn error(err_stt: ErrorStatus) -> Self {
        let msg = format!(
            "{{ error: '{}', code: {} }}",
            err_stt.text(),
            err_stt.code()
        )
        .into_bytes();

        Self::Error {
            failure: Err(msg),
            status: err_stt,
        }
    }

    pub fn redirect(status: Status, route: &'a str) -> Self {
        Self::Redirect { status, route }
    }

    pub fn is_success(&self) -> bool {
        match self {
            Self::Success { .. } => true,
            _ => false,
        }
    }
}

pub struct TakeRequest<'a> {
    request: Request,
    service: &'a Service,
    status: Status,
}

impl<'a> TakeRequest<'a> {
    pub fn new(find_service: FindService<'a>, request: Request) -> Self {
        let FindService::Success { service, status } = find_service else {
            unreachable!("already ruled the other variants out");
        };

        Self {
            request,
            service,
            status,
        }
    }
}
//
// pub struct Respondent<Meta> {
//     proto: Protocol,
//     cookies: HashSet<Cookie>,
// }
//
// pub struct Respondent<Data> {
//     status: Status,
//     body: Option<Vec<u8>>,
//     headers: HashMap<String, String>,
//     proto: Protocol,
//     cookies: HashSet<Cookie>,
// }
//
// impl Response {
//     fn inscribe_meta(req: Request) -> Self {
//         Self { proto: req.proto() }
//     }
// }
