// TODO this module is todo

use super::{
    ClientError, Cors, ErrorStatus, Fallback, FindProcess, GoodStatus, HttpSocket, Method,
    PheasantError, PheasantResult, Process, Protocol, Redirection, Request, Respond, Response,
    ResponseStatus, ServerError, Status, Successful, TakeRequest,
};
use std::io::{BufRead, Write};
use std::net::{Ipv4Addr, TcpStream};

// the HttpLoop trait
// 0 -> client makes request through tcp socket
// 1 -> server (we) receive request bytes in the stream
// 2 -> server reads request bytes into a request instance // trait method
// 3 -> server looks at request line
//  3.1 -> is method supported? // trait method
//  3.2 -> is there a service registered with the resource + method? // trait method
//  3.3 -> is the resource registered as a redirect to an available service // trait method
//  3.4 -> server decides what to do with the request // trait method
// 4 -> server looks at request headers and body and handles them in conjunction with the req line data
//  4.1 -> there are X types of headers:
//      a -> meta headers: they decribe the server or the client
//      b -> method headers: they implement some logic that depends upon the req method
//      c -> body headers: related to the req body
// 5 -> server generates response status line + headers + body
//  from the server wide configs + service configs + the request itself
// 6 -> server sends response to the client
// done

// TODO request parsing should be split into multiple functions and they should be called
// as/when needed
// i.e., if you parse request line and the server doesnt know of the resource in it, then we
// simply early return a 404 not found
// => potentially, request should be a trait not a struct

// TODO add support for HEAD method: HEAD is GET but with headers only / with no request body
// TODO macro attr #[head]

// TODO user defined http statusin service macros

pub trait ProcessClassifier {
    /// returns all the registered services of a method
    fn method_services(&self, method: Method) -> impl Iterator<Item = &Process>;

    /// returns all the allowed methods of a resource
    fn resource_methods(&self, route: &str) -> impl Iterator<Item = Method>;

    fn service_variants(&self, route: &str) -> impl Iterator<Item = &Process>;
}

impl ProcessClassifier for HttpSocket {
    fn method_services(&self, method: Method) -> impl Iterator<Item = &Process> {
        self.services_iter().filter(move |s| s.method() == method)
    }

    fn resource_methods(&self, route: &str) -> impl Iterator<Item = Method> {
        self.services_iter()
            .filter(move |s| s.route() == route)
            .map(|s| s.method())
    }

    fn service_variants(&self, route: &str) -> impl Iterator<Item = &Process> {
        // TODO
        self.services_iter().filter(move |s| s.route() == route)
    }
}

// this needs to support different protocols of http as well as a wrapper tls tunnel
pub trait HttpBackAndForth {
    // returns the service matching the req method + route if it exists
    //
    // else returns the corresponding http error status
    fn find_service(&self, method: Method, route: &str) -> Result<FindProcess, ErrorStatus>;

    // returns the corresponding failure service for this http error if it exists
    //
    // else returns None
    fn http_error(&self, status: ErrorStatus) -> Option<&Fallback>;

    fn http_error_fallback(&self, err_stt: ErrorStatus) -> FindProcess {
        match self.http_error(err_stt) {
            Some(fail) => FindProcess::Error {
                failure: Ok(fail),
                status: err_stt,
            },
            None => FindProcess::error(err_stt),
        }
    }

    fn responder(&self, method: Method, route: &str) -> FindProcess {
        match self.find_service(method, route) {
            Ok(resp) => resp,
            Err(err_stt) => self.http_error_fallback(err_stt),
        }
    }
}

// F is the function that reads and parses the http 1.1 request
impl HttpBackAndForth for HttpSocket {
    fn find_service(&self, method: Method, route: &str) -> Result<FindProcess, ErrorStatus> {
        if !self.services_iter().any(|s| s.method() == method) {
            // from `https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/501`
            // 501 is the appropriate response when the server does not recognize the request
            // method and is incapable of supporting it for any resource
            return Err(ErrorStatus::Server(ServerError::NotImplemented));
        }

        if let Some(service) = self
            .services_iter()
            .filter(|s| s.method() == method)
            .find(|s| s.route() == route)
        {
            Ok(FindProcess::success(service, method))
        } else if let Some(service) = self
            .services_iter()
            .filter(|s| s.method() == method)
            .find(|s| s.redirects_to(route))
        {
            // TODO if many services have the same redirection + same method but different route
            // then we return redirect 300 multiple choices instead of the first
            // TODO service macro attr: #[variants(lang1, lang2)]
            // generates a service for each variant + sets a redirect for all of them from the
            // original service route
            Ok(FindProcess::redirect(
                Status::Redirection(Redirection::SeeOther),
                service.route(),
            ))
        } else {
            Err(ErrorStatus::Client(ClientError::MethodNotAllowed))
        }
    }

    fn http_error(&self, err_stt: ErrorStatus) -> Option<&Fallback> {
        self.failures_iter().find(|f| f.code() == err_stt.code())
    }
}

impl HttpSocket {
    /// searches for the specified service
    /// returns `(Status, &Process)`
    ///
    /// ### Error
    /// returns an Err, a client error (404 not found) if the service is not found
    pub fn service_status(
        &self,
        method: Method,
        route: &str,
    ) -> PheasantResult<(Status, &Process)> {
        // FIXME this suddenly broke after implementing Hash, Eq, PartialEq on Process
        match self
            .services_iter()
            // .inspect(|s| println!("{}:{:?}", s.route(), s.re()))
            .find(|s| {
                if method == Method::Options {
                    s.method() == Method::Options
                        && (s.route() == route
                            || self.services_iter().any(|s| s.redirects_to(route)))
                } else {
                    s.method() == method && (s.route() == route || s.redirects_to(&route))
                }
            }) {
            Some(ref s) if s.route() == route => Ok((Status::Successful(Successful::OK), s)),
            Some(ref s) if s.redirects_to(&route) => {
                Ok((Status::Redirection(Redirection::SeeOther), s))
            }
            Some(ref s) if s.method() == Method::Options => {
                Ok((Status::Successful(Successful::NoContent), s))
            }
            None => Err(PheasantError::ClientError(ClientError::NotFound)),
            Some(_) => unreachable!(
                "logic break: the Process that matches the conditions didnt match the condititons"
            ),
        }
    }

    /// searches for the speficied `Fail` (error status fallback service)
    /// returns `Some(&Fail)` if found
    /// else returns `None`
    pub fn fail_status(&self, status_code: u16) -> Option<&Fallback> {
        self.failures_iter().find(move |e| e.code() == status_code)
    }

    /// launch the service
    /// listening for incoming tcp streams
    /// and handling them
    pub async fn serve(&mut self) {
        for stream in self.socket_ref().incoming().flatten() {
            if let Err(e) = self.handle_stream(stream).await {
                // TODO log the error or something
                println!("{:?}", e);
            }
        }
    }

    // handles a tcp stream connection
    async fn handle_stream(&self, mut stream: TcpStream) -> PheasantResult<TcpStream> {
        let req = Request::from_stream(&mut stream);
        println!("{:#?}\n", req); // if req is err we return a status error response
        let Ok(req) = req else {
            let resp = self.error_template(400, None).await;

            return send_response(stream, resp);
        };

        let resp = match self.service_status(req.method(), req.route()) {
            Ok((status, service)) => Response::payload(req, status, service).await,
            Err(PheasantError::ClientError(ClientError::NotFound)) => {
                self.error_template(404, Some(req.proto())).await
            }
            _ => unimplemented!("not implemented yet"),
        };

        send_response(stream, resp)
    }

    // TODO this and Response::from_err have become redundant since (Fallback.callback)() now returns
    // a Response
    // TODO fix Response mess
    pub async fn error_template(&self, code: u16, proto: Option<Protocol>) -> Response {
        let fail = self.fail_status(code);
        Response::from_err(fail, proto)
            .await
            .unwrap_or(Response::not_implemented().await)
    }
}

// sends the response to the client and returns the connection tcp stream
fn send_response<W: Write>(mut stream: W, resp: Response) -> PheasantResult<W> {
    let payload = resp.respond();

    stream.write_all(&payload)?;
    stream.flush()?;

    Ok(stream)
}
