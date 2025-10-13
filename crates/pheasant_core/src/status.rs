use crate::PheasantError;
use alloc::string::ToString;
use core::fmt::{self, Debug};
use core::str::FromStr;

#[macro_export]
macro_rules! status {
    ($var: ident) => {
        stringify!($var).parse::<Status>().unwrap()
    };
    ($code: expr) => {
        Status::try_from($code).unwrap()
    };
}

#[macro_export]
macro_rules! err_stt {
    ($var: ident) => {
        stringify!($var).parse::<$crate::ErrorStatus>().unwrap()
    };
    ($code: expr) => {
        $crate::ErrorStatus::try_from($code).unwrap()
    };
    (? $var: ident) => {
        Err(stringify!($var).parse::<$crate::ErrorStatus>().unwrap())
    };
    (? $code: expr) => {
        Err($crate::ErrorStatus::try_from($code).unwrap())
    };
}

// #[macro_export]
// macro_rules! s_err {
//     ($var: ident) => {
//         stringify!($var)
//             .parse::<pheasant_core::ErrorStatus>()
//             .unwrap()
//     };
//     ($code: expr) => {
//         pheasant_core::ErrorStatus::try_from($code).unwrap()
//     };
// }

// #[macro_export]
// macro_rules! c_err {
//     ($var: ident) => {
//         stringify!($var)
//             .parse::<pheasant_core::ErrorStatus>()
//             .unwrap()
//     };
//     ($code: expr) => {
//         pheasant_core::ErrorStatus::try_from($code).unwrap()
//     };
// }

macro_rules! status_enum {
     ($name: ident, $($var: ident $code: literal),*) => {
        #[repr(u16)]
        #[derive(Debug, Clone, Copy, PartialEq,
            Eq, Hash, serde::Serialize, serde::Deserialize)]
         pub enum $name {$(
             $var = $code,
         )*}

     impl $name {
         pub fn str_lit(&self) -> &'static str {
             match self {
                 $(Self :: $var => stringify!($name::$var),)*
             }
         }
     }

     impl std::str::FromStr for $name {
         type Err = ();

         fn from_str(s: &str) -> Result<Self,Self::Err> {
             match s {
                 $(stringify!($var) => Ok($name :: $var),)*
                 _ => Err(()),
             }
         }
     }

    impl TryFrom<u16> for $name {
         type Error = ();

         fn try_from(u: u16) -> Result<Self, Self::Error> {
            match u {
                $($code => Ok(Self:: $var), )*
                _ => Err(()),
             }
         }
     }

     };
 }

/// http response server error status,
status_enum!(ServerError,
    InternalServerError 500,
    NotImplemented 501,
    BadGateway 502,
    ProcessUnavailable 503,
    GatewayTimeout 504,
    HTTPVersionNotSupported 505,
    VariantAlsoNegotiates 506,
    InsufficientStorage 507,
    LoopDetected 508,
    NotExtended 510,
    NetworkAuthenticationRequired 511
);

/// http response client error status
status_enum!(ClientError,
    BadRequest 400,
    Unauthorized 401,
    // NOTE rarely used
    PaymentRequired 402,
    Forbidden 403,
    NotFound 404,
    MethodNotAllowed 405,
    NotAcceptable 406,
    ProxyAuthenticationRequired 407,
    RequestTimeout 408,
    Conflict 409,
    Gone 410,
    LengthRequired 411,
    PreconditionFailed 412,
    ContentTooLarge 413,
    URITooLong 414,
    UnsupportedMediaType 415,
    RangeNotSatisfiable 416,
    ExpectationFailed 417,
    Imateapot 418,
    MisdirectedRequest 421,
    UnprocessableContent 422,
    Locked 423,
    FailedDependency 424,
    TooEarly 425,
    UpgradeRequired 426,
    PreconditionRequired 428,
    TooManyRequests 429,
    RequestHeaderFieldsTooLarge 431,
    UnavailableForLegalReasons 451
);

// #[deprecated(
//     note = "This response code is no longer used; but is reserved. It was used in a previous version of the HTTP/1.1 specification.")]
// Unused 306,
// #[deprecated(
//     note = "deprecated due to security concerns regarding in-band configuration of a proxy.")]
// /// Defined in a previous version of the HTTP specification
// /// to indicate that a requested response must be accessed by a proxy
// UseProxyDeprecated 305,

/// http response redirection status
status_enum!(Redirection,
    PermanentRedirect 308,
    TemporaryRedirect 307,
    Unused 306,
    UseProxyDeprecated 305,
    NotModified 304,
    SeeOther 303,
    Found 302,
    MovedPermanently 301,
    MultipleChoices 300
);

/// http response successful status
status_enum!(Successful,
    IMUsed 226,
    AlreadyReported 208,
    MultiStatus 207,
    PartialContent 206,
    ResetContent 205,
    NoContent 204,
    NonAuthoritativeInformation 203,
    Accepted 202,
    Created 201,
    OK 200
);

/// http response informational status
status_enum!(Informational,
    EarlyHints 103,
    ProcessingDeprecated 102,
    SwitchingProtocols 101,
    Continue 100
);

impl From<PheasantError> for Status {
    fn from(err: PheasantError) -> Self {
        match err {
            PheasantError::ClientError(ce) => Self::ClientError(ce),

            PheasantError::ServerError(se) => Self::ServerError(se),
        }
    }
}

/// implements shared behavior amongst response status
pub trait StatusLiterals {
    /// returns the status text value
    fn text(&self) -> &str;

    /// returns the status code number
    fn code(&self) -> u16;
}

impl StatusLiterals for ServerError {
    fn text(&self) -> &str {
        match self {
            Self::InternalServerError => "InternalServerError",
            Self::NotImplemented => "NotImplemented",
            Self::BadGateway => "BadGateway",
            Self::ProcessUnavailable => "ProcessUnavailable",
            Self::GatewayTimeout => "GatewayTimeout",
            Self::HTTPVersionNotSupported => "HTTPVersionNotSupported",
            Self::VariantAlsoNegotiates => "VariantAlsoNegotiates",
            Self::InsufficientStorage => "InsufficientStorage",
            Self::LoopDetected => "LoopDetected",
            Self::NotExtended => "NotExtended",
            Self::NetworkAuthenticationRequired => "NetworkAuthenticationRequired",
        }
    }

    fn code(&self) -> u16 {
        unsafe { core::mem::transmute::<Self, u16>(*self) }
    }
}

impl StatusLiterals for ClientError {
    fn text(&self) -> &str {
        match self {
            Self::BadRequest => "BadRequest",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "PaymentRequired",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "NotFound",
            Self::MethodNotAllowed => "MethodNotAllowed",
            Self::NotAcceptable => "NotAcceptable",
            Self::ProxyAuthenticationRequired => "ProxyAuthenticationRequired",
            Self::RequestTimeout => "RequestTimeout",
            Self::Conflict => "Conflict",
            Self::Gone => "Gone",
            Self::LengthRequired => "LengthRequired",
            Self::PreconditionFailed => "PreconditionFailed",
            Self::ContentTooLarge => "ContentTooLarge",
            Self::URITooLong => "URITooLong",
            Self::UnsupportedMediaType => "UnsupportedMediaType",
            Self::RangeNotSatisfiable => "RangeNotSatisfiable",
            Self::ExpectationFailed => "ExpectationFailed",
            Self::Imateapot => "Imateapot",
            Self::MisdirectedRequest => "MisdirectedRequest",
            Self::UnprocessableContent => "UnprocessableContent",
            Self::Locked => "Locked",
            Self::FailedDependency => "FailedDependency",
            Self::TooEarly => "TooEarly",
            Self::UpgradeRequired => "UpgradeRequired",
            Self::PreconditionRequired => "PreconditionRequired",
            Self::TooManyRequests => "TooManyRequests",
            Self::RequestHeaderFieldsTooLarge => "RequestHeaderFieldsTooLarge",
            Self::UnavailableForLegalReasons => "UnavailableForLegalReasons",
        }
    }

    fn code(&self) -> u16 {
        unsafe { core::mem::transmute::<Self, u16>(*self) }
    }
}

impl StatusLiterals for Redirection {
    fn text(&self) -> &str {
        match self {
            Self::PermanentRedirect => "PermanentRedirect",
            Self::TemporaryRedirect => "TemporaryRedirect",
            Self::Unused => "Unused",
            Self::UseProxyDeprecated => "UseProxyDeprecated",
            Self::NotModified => "NotModified",
            Self::SeeOther => "SeeOther",
            Self::Found => "Found",
            Self::MovedPermanently => "MovedPermanently",
            Self::MultipleChoices => "MultipleChoices",
        }
    }

    fn code(&self) -> u16 {
        unsafe { core::mem::transmute::<Self, u16>(*self) }
    }
}

impl StatusLiterals for Successful {
    fn text(&self) -> &str {
        match self {
            Self::IMUsed => "IMUsed",
            Self::AlreadyReported => "AlreadyReported",
            Self::MultiStatus => "MultiStatus",
            Self::PartialContent => "PartialContent",
            Self::ResetContent => "ResetContent",
            Self::NoContent => "NoContent",
            Self::NonAuthoritativeInformation => "NonAuthoritativeInformation",
            Self::Accepted => "Accepted",
            Self::Created => "Created",
            Self::OK => "OK",
        }
    }

    fn code(&self) -> u16 {
        // NOTE this is safe
        // trust me bro
        unsafe { core::mem::transmute::<Self, u16>(*self) }
    }
}

impl StatusLiterals for Informational {
    fn text(&self) -> &str {
        match self {
            Self::EarlyHints => "EarlyHints",
            Self::ProcessingDeprecated => "ProcessingDeprecated",
            Self::SwitchingProtocols => "SwitchingProtocols",
            Self::Continue => "Continue",
        }
    }

    fn code(&self) -> u16 {
        unsafe { core::mem::transmute::<Self, u16>(*self) }
    }
}

/// enum wrapping all response status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Status {
    Informational(Informational),
    Successful(Successful),
    Redirection(Redirection),
    ClientError(ClientError),
    ServerError(ServerError),
}

impl Status {
    pub fn is_informational(&self) -> bool {
        let Self::Informational(_) = self else {
            return false;
        };

        true
    }

    pub fn is_successful(&self) -> bool {
        let Self::Successful(_) = self else {
            return false;
        };

        true
    }

    pub fn is_redirection(&self) -> bool {
        let Self::Redirection(_) = self else {
            return false;
        };

        true
    }

    pub fn is_client_error(&self) -> bool {
        let Self::ClientError(_) = self else {
            return false;
        };

        true
    }

    pub fn is_server_error(&self) -> bool {
        let Self::ServerError(_) = self else {
            return false;
        };

        true
    }
}

impl Default for Status {
    fn default() -> Self {
        status!(Ok)
    }
}

impl FromStr for Status {
    type Err = ();
    fn from_str(text: &str) -> Result<Self, ()> {
        match text {
            "EarlyHints" => Ok(Self::Informational(Informational::EarlyHints)),
            "ProcessingDeprecated" => Ok(Self::Informational(Informational::ProcessingDeprecated)),
            "SwitchingProtocols" => Ok(Self::Informational(Informational::SwitchingProtocols)),
            "Continue" => Ok(Self::Informational(Informational::Continue)),

            "IMUsed" => Ok(Self::Successful(Successful::IMUsed)),
            "AlreadyReported" => Ok(Self::Successful(Successful::AlreadyReported)),
            "MultiStatus" => Ok(Self::Successful(Successful::MultiStatus)),
            "PartialContent" => Ok(Self::Successful(Successful::PartialContent)),
            "ResetContent" => Ok(Self::Successful(Successful::ResetContent)),
            "NoContent" => Ok(Self::Successful(Successful::NoContent)),
            "NonAuthoritativeInformation" => {
                Ok(Self::Successful(Successful::NonAuthoritativeInformation))
            }
            "Accepted" => Ok(Self::Successful(Successful::Accepted)),
            "Created" => Ok(Self::Successful(Successful::Created)),
            "OK" => Ok(Self::Successful(Successful::OK)),

            "PermanentRedirect" => Ok(Self::Redirection(Redirection::PermanentRedirect)),
            "TemporaryRedirect" => Ok(Self::Redirection(Redirection::TemporaryRedirect)),
            "Unused" => Ok(Self::Redirection(Redirection::Unused)),
            "UseProxyDeprecated" => Ok(Self::Redirection(Redirection::UseProxyDeprecated)),
            "NotModified" => Ok(Self::Redirection(Redirection::NotModified)),
            "SeeOther" => Ok(Self::Redirection(Redirection::SeeOther)),
            "Found" => Ok(Self::Redirection(Redirection::Found)),
            "MovedPermanently" => Ok(Self::Redirection(Redirection::MovedPermanently)),
            "MultipleChoices" => Ok(Self::Redirection(Redirection::MultipleChoices)),

            "BadRequest" => Ok(Self::ClientError(ClientError::BadRequest)),
            "Unauthorized" => Ok(Self::ClientError(ClientError::Unauthorized)),
            "PaymentRequired" => Ok(Self::ClientError(ClientError::PaymentRequired)),
            "Forbidden" => Ok(Self::ClientError(ClientError::Forbidden)),
            "NotFound" => Ok(Self::ClientError(ClientError::NotFound)),
            "MethodNotAllowed" => Ok(Self::ClientError(ClientError::MethodNotAllowed)),
            "NotAcceptable" => Ok(Self::ClientError(ClientError::NotAcceptable)),
            "ProxyAuthenticationRequired" => {
                Ok(Self::ClientError(ClientError::ProxyAuthenticationRequired))
            }
            "RequestTimeout" => Ok(Self::ClientError(ClientError::RequestTimeout)),
            "Conflict" => Ok(Self::ClientError(ClientError::Conflict)),
            "Gone" => Ok(Self::ClientError(ClientError::Gone)),
            "LengthRequired" => Ok(Self::ClientError(ClientError::LengthRequired)),
            "PreconditionFailed" => Ok(Self::ClientError(ClientError::PreconditionFailed)),
            "ContentTooLarge" => Ok(Self::ClientError(ClientError::ContentTooLarge)),
            "URITooLong" => Ok(Self::ClientError(ClientError::URITooLong)),
            "UnsupportedMediaType" => Ok(Self::ClientError(ClientError::UnsupportedMediaType)),
            "RangeNotSatisfiable" => Ok(Self::ClientError(ClientError::RangeNotSatisfiable)),
            "ExpectationFailed" => Ok(Self::ClientError(ClientError::ExpectationFailed)),
            "Imateapot" => Ok(Self::ClientError(ClientError::Imateapot)),
            "MisdirectedRequest" => Ok(Self::ClientError(ClientError::MisdirectedRequest)),
            "UnprocessableContent" => Ok(Self::ClientError(ClientError::UnprocessableContent)),
            "Locked" => Ok(Self::ClientError(ClientError::Locked)),
            "FailedDependency" => Ok(Self::ClientError(ClientError::FailedDependency)),
            "TooEarly" => Ok(Self::ClientError(ClientError::TooEarly)),
            "UpgradeRequired" => Ok(Self::ClientError(ClientError::UpgradeRequired)),
            "PreconditionRequired" => Ok(Self::ClientError(ClientError::PreconditionRequired)),
            "TooManyRequests" => Ok(Self::ClientError(ClientError::TooManyRequests)),
            "RequestHeaderFieldsTooLarge" => {
                Ok(Self::ClientError(ClientError::RequestHeaderFieldsTooLarge))
            }
            "UnavailableForLegalReasons" => {
                Ok(Self::ClientError(ClientError::UnavailableForLegalReasons))
            }

            "InternalServerError" => Ok(Self::ServerError(ServerError::InternalServerError)),
            "NotImplemented" => Ok(Self::ServerError(ServerError::NotImplemented)),
            "BadGateway" => Ok(Self::ServerError(ServerError::BadGateway)),
            "ProcessUnavailable" => Ok(Self::ServerError(ServerError::ProcessUnavailable)),
            "GatewayTimeout" => Ok(Self::ServerError(ServerError::GatewayTimeout)),
            "HTTPVersionNotSupported" => {
                Ok(Self::ServerError(ServerError::HTTPVersionNotSupported))
            }
            "VariantAlsoNegotiates" => Ok(Self::ServerError(ServerError::VariantAlsoNegotiates)),
            "InsufficientStorage" => Ok(Self::ServerError(ServerError::InsufficientStorage)),
            "LoopDetected" => Ok(Self::ServerError(ServerError::LoopDetected)),
            "NotExtended" => Ok(Self::ServerError(ServerError::NotExtended)),
            "NetworkAuthenticationRequired" => Ok(Self::ServerError(
                ServerError::NetworkAuthenticationRequired,
            )),
            _ => Err(()),
        }
    }
}

impl TryFrom<u16> for Status {
    type Error = ();

    fn try_from(u: u16) -> Result<Self, Self::Error> {
        if u < 200 {
            <u16 as TryInto<Informational>>::try_into(u).map(|i| Self::Informational(i))
        } else if u < 300 {
            <u16 as TryInto<Successful>>::try_into(u).map(|s| Self::Successful(s))
        } else if u < 400 {
            <u16 as TryInto<Redirection>>::try_into(u).map(|re| Self::Redirection(re))
        } else if u < 500 {
            <u16 as TryInto<ClientError>>::try_into(u).map(|ce| Self::ClientError(ce))
        } else {
            <u16 as TryInto<ServerError>>::try_into(u).map(|se| Self::ServerError(se))
        }
    }
}

impl StatusLiterals for Status {
    fn text(&self) -> &str {
        match self {
            Self::Redirection(r) => r.text(),
            Self::Successful(s) => s.text(),
            Self::Informational(i) => i.text(),
            Self::ClientError(ce) => ce.text(),
            Self::ServerError(se) => se.text(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            Self::Redirection(r) => r.code(),
            Self::Successful(s) => s.code(),
            Self::Informational(i) => i.code(),
            Self::ClientError(ce) => ce.code(),
            Self::ServerError(se) => se.code(),
        }
    }
}

/// request accpetance response status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PassingStatus {
    Redirection(Redirection),
    Informational(Informational),
    Successful(Successful),
}

impl StatusLiterals for PassingStatus {
    fn text(&self) -> &str {
        match self {
            Self::Informational(i) => i.text(),
            Self::Successful(s) => s.text(),
            Self::Redirection(re) => re.text(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            Self::Informational(i) => i.code(),
            Self::Successful(s) => s.code(),
            Self::Redirection(re) => re.code(),
        }
    }
}

// enum Status {
//     Reject(ErrorStatus),
//     Accept(AcceptStatus),
// }

/// error response status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ErrorStatus {
    Client(ClientError),
    Server(ServerError),
}

impl ErrorStatus {
    fn is_server_err(&self) -> bool {
        let Self::Server(_) = self else { return false };

        true
    }

    fn is_client_err(&self) -> bool {
        let Self::Client(_) = self else { return false };

        true
    }
}

impl StatusLiterals for ErrorStatus {
    fn text(&self) -> &str {
        match self {
            Self::Client(ce) => ce.text(),
            Self::Server(se) => se.text(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            Self::Client(ce) => ce.code(),
            Self::Server(se) => se.code(),
        }
    }
}

// WARN this is wrong
// if u == unrecognizable variant repr, then this fn returns the highest u16 repr variant
impl TryFrom<u16> for ErrorStatus {
    type Error = ();

    fn try_from(u: u16) -> Result<Self, Self::Error> {
        if u < 500 {
            <u16 as TryInto<ClientError>>::try_into(u).map(|ce| Self::Client(ce))
        } else {
            <u16 as TryInto<ServerError>>::try_into(u).map(|se| Self::Server(se))
        }
    }
}

impl FromStr for ErrorStatus {
    type Err = ();
    fn from_str(text: &str) -> Result<Self, ()> {
        match text {
            "BadRequest" => Ok(Self::Client(ClientError::BadRequest)),
            "Unauthorized" => Ok(Self::Client(ClientError::Unauthorized)),
            "PaymentRequired" => Ok(Self::Client(ClientError::PaymentRequired)),
            "Forbidden" => Ok(Self::Client(ClientError::Forbidden)),
            "NotFound" => Ok(Self::Client(ClientError::NotFound)),
            "MethodNotAllowed" => Ok(Self::Client(ClientError::MethodNotAllowed)),
            "NotAcceptable" => Ok(Self::Client(ClientError::NotAcceptable)),
            "ProxyAuthenticationRequired" => {
                Ok(Self::Client(ClientError::ProxyAuthenticationRequired))
            }
            "RequestTimeout" => Ok(Self::Client(ClientError::RequestTimeout)),
            "Conflict" => Ok(Self::Client(ClientError::Conflict)),
            "Gone" => Ok(Self::Client(ClientError::Gone)),
            "LengthRequired" => Ok(Self::Client(ClientError::LengthRequired)),
            "PreconditionFailed" => Ok(Self::Client(ClientError::PreconditionFailed)),
            "ContentTooLarge" => Ok(Self::Client(ClientError::ContentTooLarge)),
            "URITooLong" => Ok(Self::Client(ClientError::URITooLong)),
            "UnsupportedMediaType" => Ok(Self::Client(ClientError::UnsupportedMediaType)),
            "RangeNotSatisfiable" => Ok(Self::Client(ClientError::RangeNotSatisfiable)),
            "ExpectationFailed" => Ok(Self::Client(ClientError::ExpectationFailed)),
            "Imateapot" => Ok(Self::Client(ClientError::Imateapot)),
            "MisdirectedRequest" => Ok(Self::Client(ClientError::MisdirectedRequest)),
            "UnprocessableContent" => Ok(Self::Client(ClientError::UnprocessableContent)),
            "Locked" => Ok(Self::Client(ClientError::Locked)),
            "FailedDependency" => Ok(Self::Client(ClientError::FailedDependency)),
            "TooEarly" => Ok(Self::Client(ClientError::TooEarly)),
            "UpgradeRequired" => Ok(Self::Client(ClientError::UpgradeRequired)),
            "PreconditionRequired" => Ok(Self::Client(ClientError::PreconditionRequired)),
            "TooManyRequests" => Ok(Self::Client(ClientError::TooManyRequests)),
            "RequestHeaderFieldsTooLarge" => {
                Ok(Self::Client(ClientError::RequestHeaderFieldsTooLarge))
            }
            "UnavailableForLegalReasons" => {
                Ok(Self::Client(ClientError::UnavailableForLegalReasons))
            }

            "InternalServerError" => Ok(Self::Server(ServerError::InternalServerError)),
            "NotImplemented" => Ok(Self::Server(ServerError::NotImplemented)),
            "BadGateway" => Ok(Self::Server(ServerError::BadGateway)),
            "ProcessUnavailable" => Ok(Self::Server(ServerError::ProcessUnavailable)),
            "GatewayTimeout" => Ok(Self::Server(ServerError::GatewayTimeout)),
            "HTTPVersionNotSupported" => Ok(Self::Server(ServerError::HTTPVersionNotSupported)),
            "VariantAlsoNegotiates" => Ok(Self::Server(ServerError::VariantAlsoNegotiates)),
            "InsufficientStorage" => Ok(Self::Server(ServerError::InsufficientStorage)),
            "LoopDetected" => Ok(Self::Server(ServerError::LoopDetected)),
            "NotExtended" => Ok(Self::Server(ServerError::NotExtended)),
            "NetworkAuthenticationRequired" => {
                Ok(Self::Server(ServerError::NetworkAuthenticationRequired))
            }
            _ => Err(()),
        }
    }
}

impl From<ErrorStatus> for u16 {
    fn from(err: ErrorStatus) -> u16 {
        match err {
            ErrorStatus::Client(ce) => ce.code(),
            ErrorStatus::Server(se) => se.code(),
        }
    }
}

use proc_macro2::{Delimiter, Group, Punct, Spacing, Span, TokenStream as TS2, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{Ident, Token};

impl ErrorStatus {
    fn str_lit(&self) -> &str {
        match self {
            Self::Client(ce) => ce.str_lit(),
            Self::Server(se) => se.str_lit(),
        }
    }
}

impl ErrorStatus {
    fn str_var(&self) -> &str {
        match self {
            Self::Server(_) => "Server",
            Self::Client(_) => "Client",
        }
    }
}

impl From<ErrorStatus> for Status {
    fn from(err: ErrorStatus) -> Self {
        match err {
            ErrorStatus::Client(ce) => Self::ClientError(ce),
            ErrorStatus::Server(se) => Self::ServerError(se),
        }
    }
}

impl ToTokens for ErrorStatus {
    fn to_tokens(&self, tokens: &mut TS2) {
        tokens.append(<ErrorStatus as Into<TokenTree>>::into(*self))
    }
}

impl From<ErrorStatus> for TokenTree {
    fn from(err: ErrorStatus) -> Self {
        let [var, subtype, subvar] = {
            let s = err.to_string();
            let mut iter = s
                .split("::")
                .map(|s| Ident::new(s.trim(), Span::call_site()));

            [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]
        };

        Group::new(
            Delimiter::None,
            quote::quote! { ErrorStatus::#var(pheasant:: #subtype::#subvar) },
        )
        .into()
    }
}

impl fmt::Display for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.str_var(), self.str_lit())
    }
}
