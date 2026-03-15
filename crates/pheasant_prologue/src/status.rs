use crate::PheasantError;
use alloc::string::ToString;
use core::fmt::{self, Debug};
use core::str::FromStr;

#[macro_export]
macro_rules! status {
    ($var: ident) => {
        stringify!($var).parse::<$crate::Status>().unwrap()
    };
    ($code: expr) => {
        $crate::Status::try_from($code).unwrap()
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl ServerError {
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

    /// nothing really unsafe going on here
    fn code(&self) -> u16 {
        unsafe { core::mem::transmute::<Self, u16>(*self) }
    }
}

impl ClientError {
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

impl Redirection {
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

impl Successful {
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

impl Informational {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Informational(Informational),
    Successful(Successful),
    Redirection(Redirection),
    ClientError(ClientError),
    ServerError(ServerError),
}

impl Status {
    pub fn text(&self) -> &str {
        match self {
            Self::Redirection(r) => r.text(),
            Self::Successful(s) => s.text(),
            Self::Informational(i) => i.text(),
            Self::ClientError(ce) => ce.text(),
            Self::ServerError(se) => se.text(),
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            Self::Redirection(r) => r.code(),
            Self::Successful(s) => s.code(),
            Self::Informational(i) => i.code(),
            Self::ClientError(ce) => ce.code(),
            Self::ServerError(se) => se.code(),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        status!(Ok)
    }
}

impl FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "EarlyHints" | "103" => Ok(Self::Informational(Informational::EarlyHints)),
            "ProcessingDeprecated" | "102" => {
                Ok(Self::Informational(Informational::ProcessingDeprecated))
            }
            "SwitchingProtocols" | "101" => {
                Ok(Self::Informational(Informational::SwitchingProtocols))
            }
            "Continue" | "100" => Ok(Self::Informational(Informational::Continue)),

            "IMUsed" | "226" => Ok(Self::Successful(Successful::IMUsed)),
            "AlreadyReported" | "208" => Ok(Self::Successful(Successful::AlreadyReported)),
            "MultiStatus" | "207" => Ok(Self::Successful(Successful::MultiStatus)),
            "PartialContent" | "206" => Ok(Self::Successful(Successful::PartialContent)),
            "ResetContent" | "205" => Ok(Self::Successful(Successful::ResetContent)),
            "NoContent" | "204" => Ok(Self::Successful(Successful::NoContent)),
            "NonAuthoritativeInformation" | "203" => {
                Ok(Self::Successful(Successful::NonAuthoritativeInformation))
            }
            "Accepted" | "202" => Ok(Self::Successful(Successful::Accepted)),
            "Created" | "201" => Ok(Self::Successful(Successful::Created)),
            "OK" | "200" => Ok(Self::Successful(Successful::OK)),

            "PermanentRedirect" | "308" => Ok(Self::Redirection(Redirection::PermanentRedirect)),
            "TemporaryRedirect" | "307" => Ok(Self::Redirection(Redirection::TemporaryRedirect)),
            "Unused" | "306" => Ok(Self::Redirection(Redirection::Unused)),
            "UseProxyDeprecated" | "305" => Ok(Self::Redirection(Redirection::UseProxyDeprecated)),
            "NotModified" | "304" => Ok(Self::Redirection(Redirection::NotModified)),
            "SeeOther" | "303" => Ok(Self::Redirection(Redirection::SeeOther)),
            "Found" | "302" => Ok(Self::Redirection(Redirection::Found)),
            "MovedPermanently" | "301" => Ok(Self::Redirection(Redirection::MovedPermanently)),
            "MultipleChoices" | "300" => Ok(Self::Redirection(Redirection::MultipleChoices)),

            "BadRequest" | "400" => Ok(Self::ClientError(ClientError::BadRequest)),
            "Unauthorized" | "401" => Ok(Self::ClientError(ClientError::Unauthorized)),
            "PaymentRequired" | "402" => Ok(Self::ClientError(ClientError::PaymentRequired)),
            "Forbidden" | "403" => Ok(Self::ClientError(ClientError::Forbidden)),
            "NotFound" | "404" => Ok(Self::ClientError(ClientError::NotFound)),
            "MethodNotAllowed" | "405" => Ok(Self::ClientError(ClientError::MethodNotAllowed)),
            "NotAcceptable" | "406" => Ok(Self::ClientError(ClientError::NotAcceptable)),
            "ProxyAuthenticationRequired" | "407" => {
                Ok(Self::ClientError(ClientError::ProxyAuthenticationRequired))
            }
            "RequestTimeout" | "408" => Ok(Self::ClientError(ClientError::RequestTimeout)),
            "Conflict" | "409" => Ok(Self::ClientError(ClientError::Conflict)),
            "Gone" | "410" => Ok(Self::ClientError(ClientError::Gone)),
            "LengthRequired" | "411" => Ok(Self::ClientError(ClientError::LengthRequired)),
            "PreconditionFailed" | "412" => Ok(Self::ClientError(ClientError::PreconditionFailed)),
            "ContentTooLarge" | "413" => Ok(Self::ClientError(ClientError::ContentTooLarge)),
            "URITooLong" | "414" => Ok(Self::ClientError(ClientError::URITooLong)),
            "UnsupportedMediaType" | "415" => {
                Ok(Self::ClientError(ClientError::UnsupportedMediaType))
            }
            "RangeNotSatisfiable" | "416" => {
                Ok(Self::ClientError(ClientError::RangeNotSatisfiable))
            }
            "ExpectationFailed" | "417" => Ok(Self::ClientError(ClientError::ExpectationFailed)),
            "Imateapot" | "418" => Ok(Self::ClientError(ClientError::Imateapot)),
            "MisdirectedRequest" | "421" => Ok(Self::ClientError(ClientError::MisdirectedRequest)),
            "UnprocessableContent" | "422" => {
                Ok(Self::ClientError(ClientError::UnprocessableContent))
            }
            "Locked" | "423" => Ok(Self::ClientError(ClientError::Locked)),
            "FailedDependency" | "424" => Ok(Self::ClientError(ClientError::FailedDependency)),
            "TooEarly" | "425" => Ok(Self::ClientError(ClientError::TooEarly)),
            "UpgradeRequired" | "426" => Ok(Self::ClientError(ClientError::UpgradeRequired)),
            "PreconditionRequired" | "428" => {
                Ok(Self::ClientError(ClientError::PreconditionRequired))
            }
            "TooManyRequests" | "429" => Ok(Self::ClientError(ClientError::TooManyRequests)),
            "RequestHeaderFieldsTooLarge" | "431" => {
                Ok(Self::ClientError(ClientError::RequestHeaderFieldsTooLarge))
            }
            "UnavailableForLegalReasons" | "451" => {
                Ok(Self::ClientError(ClientError::UnavailableForLegalReasons))
            }

            "InternalServerError" | "500" => {
                Ok(Self::ServerError(ServerError::InternalServerError))
            }
            "NotImplemented" | "501" => Ok(Self::ServerError(ServerError::NotImplemented)),
            "BadGateway" | "502" => Ok(Self::ServerError(ServerError::BadGateway)),
            "ProcessUnavailable" | "503" => Ok(Self::ServerError(ServerError::ProcessUnavailable)),
            "GatewayTimeout" | "504" => Ok(Self::ServerError(ServerError::GatewayTimeout)),
            "HTTPVersionNotSupported" | "505" => {
                Ok(Self::ServerError(ServerError::HTTPVersionNotSupported))
            }
            "VariantAlsoNegotiates" | "506" => {
                Ok(Self::ServerError(ServerError::VariantAlsoNegotiates))
            }
            "InsufficientStorage" | "507" => {
                Ok(Self::ServerError(ServerError::InsufficientStorage))
            }
            "LoopDetected" | "508" => Ok(Self::ServerError(ServerError::LoopDetected)),
            "NotExtended" | "510" => Ok(Self::ServerError(ServerError::NotExtended)),
            "NetworkAuthenticationRequired" | "511" => Ok(Self::ServerError(
                ServerError::NetworkAuthenticationRequired,
            )),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Status {
    type Error = ();
    fn try_from(text: &[u8]) -> Result<Self, ()> {
        match text {
            b"EarlyHints" | b"103" => Ok(Self::Informational(Informational::EarlyHints)),
            b"ProcessingDeprecated" | b"102" => {
                Ok(Self::Informational(Informational::ProcessingDeprecated))
            }
            b"SwitchingProtocols" | b"101" => {
                Ok(Self::Informational(Informational::SwitchingProtocols))
            }
            b"Continue" | b"100" => Ok(Self::Informational(Informational::Continue)),

            b"IMUsed" | b"226" => Ok(Self::Successful(Successful::IMUsed)),
            b"AlreadyReported" | b"208" => Ok(Self::Successful(Successful::AlreadyReported)),
            b"MultiStatus" | b"207" => Ok(Self::Successful(Successful::MultiStatus)),
            b"PartialContent" | b"206" => Ok(Self::Successful(Successful::PartialContent)),
            b"ResetContent" | b"205" => Ok(Self::Successful(Successful::ResetContent)),
            b"NoContent" | b"204" => Ok(Self::Successful(Successful::NoContent)),
            b"NonAuthoritativeInformation" | b"203" => {
                Ok(Self::Successful(Successful::NonAuthoritativeInformation))
            }
            b"Accepted" | b"202" => Ok(Self::Successful(Successful::Accepted)),
            b"Created" | b"201" => Ok(Self::Successful(Successful::Created)),
            b"OK" | b"200" => Ok(Self::Successful(Successful::OK)),

            b"PermanentRedirect" | b"308" => Ok(Self::Redirection(Redirection::PermanentRedirect)),
            b"TemporaryRedirect" | b"307" => Ok(Self::Redirection(Redirection::TemporaryRedirect)),
            b"Unused" | b"306" => Ok(Self::Redirection(Redirection::Unused)),
            b"UseProxyDeprecated" | b"305" => {
                Ok(Self::Redirection(Redirection::UseProxyDeprecated))
            }
            b"NotModified" | b"304" => Ok(Self::Redirection(Redirection::NotModified)),
            b"SeeOther" | b"303" => Ok(Self::Redirection(Redirection::SeeOther)),
            b"Found" | b"302" => Ok(Self::Redirection(Redirection::Found)),
            b"MovedPermanently" | b"301" => Ok(Self::Redirection(Redirection::MovedPermanently)),
            b"MultipleChoices" | b"300" => Ok(Self::Redirection(Redirection::MultipleChoices)),

            b"BadRequest" | b"400" => Ok(Self::ClientError(ClientError::BadRequest)),
            b"Unauthorized" | b"401" => Ok(Self::ClientError(ClientError::Unauthorized)),
            b"PaymentRequired" | b"402" => Ok(Self::ClientError(ClientError::PaymentRequired)),
            b"Forbidden" | b"403" => Ok(Self::ClientError(ClientError::Forbidden)),
            b"NotFound" | b"404" => Ok(Self::ClientError(ClientError::NotFound)),
            b"MethodNotAllowed" | b"405" => Ok(Self::ClientError(ClientError::MethodNotAllowed)),
            b"NotAcceptable" | b"406" => Ok(Self::ClientError(ClientError::NotAcceptable)),
            b"ProxyAuthenticationRequired" | b"407" => {
                Ok(Self::ClientError(ClientError::ProxyAuthenticationRequired))
            }
            b"RequestTimeout" | b"408" => Ok(Self::ClientError(ClientError::RequestTimeout)),
            b"Conflict" | b"409" => Ok(Self::ClientError(ClientError::Conflict)),
            b"Gone" | b"410" => Ok(Self::ClientError(ClientError::Gone)),
            b"LengthRequired" | b"411" => Ok(Self::ClientError(ClientError::LengthRequired)),
            b"PreconditionFailed" | b"412" => {
                Ok(Self::ClientError(ClientError::PreconditionFailed))
            }
            b"ContentTooLarge" | b"413" => Ok(Self::ClientError(ClientError::ContentTooLarge)),
            b"URITooLong" | b"414" => Ok(Self::ClientError(ClientError::URITooLong)),
            b"UnsupportedMediaType" | b"415" => {
                Ok(Self::ClientError(ClientError::UnsupportedMediaType))
            }
            b"RangeNotSatisfiable" | b"416" => {
                Ok(Self::ClientError(ClientError::RangeNotSatisfiable))
            }
            b"ExpectationFailed" | b"417" => Ok(Self::ClientError(ClientError::ExpectationFailed)),
            b"Imateapot" | b"418" => Ok(Self::ClientError(ClientError::Imateapot)),
            b"MisdirectedRequest" | b"421" => {
                Ok(Self::ClientError(ClientError::MisdirectedRequest))
            }
            b"UnprocessableContent" | b"422" => {
                Ok(Self::ClientError(ClientError::UnprocessableContent))
            }
            b"Locked" | b"423" => Ok(Self::ClientError(ClientError::Locked)),
            b"FailedDependency" | b"424" => Ok(Self::ClientError(ClientError::FailedDependency)),
            b"TooEarly" | b"425" => Ok(Self::ClientError(ClientError::TooEarly)),
            b"UpgradeRequired" | b"426" => Ok(Self::ClientError(ClientError::UpgradeRequired)),
            b"PreconditionRequired" | b"428" => {
                Ok(Self::ClientError(ClientError::PreconditionRequired))
            }
            b"TooManyRequests" | b"429" => Ok(Self::ClientError(ClientError::TooManyRequests)),
            b"RequestHeaderFieldsTooLarge" | b"431" => {
                Ok(Self::ClientError(ClientError::RequestHeaderFieldsTooLarge))
            }
            b"UnavailableForLegalReasons" | b"451" => {
                Ok(Self::ClientError(ClientError::UnavailableForLegalReasons))
            }

            b"InternalServerError" | b"500" => {
                Ok(Self::ServerError(ServerError::InternalServerError))
            }
            b"NotImplemented" | b"501" => Ok(Self::ServerError(ServerError::NotImplemented)),
            b"BadGateway" | b"502" => Ok(Self::ServerError(ServerError::BadGateway)),
            b"ProcessUnavailable" | b"503" => {
                Ok(Self::ServerError(ServerError::ProcessUnavailable))
            }
            b"GatewayTimeout" | b"504" => Ok(Self::ServerError(ServerError::GatewayTimeout)),
            b"HTTPVersionNotSupported" | b"505" => {
                Ok(Self::ServerError(ServerError::HTTPVersionNotSupported))
            }
            b"VariantAlsoNegotiates" | b"506" => {
                Ok(Self::ServerError(ServerError::VariantAlsoNegotiates))
            }
            b"InsufficientStorage" | b"507" => {
                Ok(Self::ServerError(ServerError::InsufficientStorage))
            }
            b"LoopDetected" | b"508" => Ok(Self::ServerError(ServerError::LoopDetected)),
            b"NotExtended" | b"510" => Ok(Self::ServerError(ServerError::NotExtended)),
            b"NetworkAuthenticationRequired" | b"511" => Ok(Self::ServerError(
                ServerError::NetworkAuthenticationRequired,
            )),
            _ => Err(()),
        }
    }
}

impl Status {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Informational(Informational::EarlyHints) => b"103 EarlyHints",
            Self::Informational(Informational::ProcessingDeprecated) => b"102 ProcessingDeprecated",
            Self::Informational(Informational::SwitchingProtocols) => b"101 SwitchingProtocols",
            Self::Informational(Informational::Continue) => b"100 Continue",

            Self::Successful(Successful::IMUsed) => b"226 IMUsed",
            Self::Successful(Successful::AlreadyReported) => b"208 AlreadyReported",
            Self::Successful(Successful::MultiStatus) => b"207 MultiStatus",
            Self::Successful(Successful::PartialContent) => b"206 PartialContent",
            Self::Successful(Successful::ResetContent) => b"205 ResetContent",
            Self::Successful(Successful::NoContent) => b"204 NoContent",
            Self::Successful(Successful::NonAuthoritativeInformation) => {
                b"203 NonAuthoritativeInformation"
            }

            Self::Successful(Successful::Accepted) => b"202 Accepted",
            Self::Successful(Successful::Created) => b"201 Created",
            Self::Successful(Successful::OK) => b"200 OK",

            Self::Redirection(Redirection::PermanentRedirect) => b"308 PermanentRedirect",
            Self::Redirection(Redirection::TemporaryRedirect) => b"307 TemporaryRedirect",
            Self::Redirection(Redirection::Unused) => b"103 Unused",
            Self::Redirection(Redirection::UseProxyDeprecated) => b"103 UseProxyDeprecated",
            Self::Redirection(Redirection::NotModified) => b"304 NotModified",
            Self::Redirection(Redirection::SeeOther) => b"303 SeeOther",
            Self::Redirection(Redirection::Found) => b"302 Found",
            Self::Redirection(Redirection::MovedPermanently) => b"301 MovedPermanently",
            Self::Redirection(Redirection::MultipleChoices) => b"300 MultipleChoices",

            Self::ClientError(ClientError::BadRequest) => b"400 BadRequest",
            Self::ClientError(ClientError::Unauthorized) => b"401 Unauthorized",
            Self::ClientError(ClientError::PaymentRequired) => b"402 PaymentRequired",
            Self::ClientError(ClientError::Forbidden) => b"403 Forbidden",
            Self::ClientError(ClientError::NotFound) => b"404 NotFound",
            Self::ClientError(ClientError::MethodNotAllowed) => b"405 MethodNotAllowed",
            Self::ClientError(ClientError::NotAcceptable) => b"406 NotAcceptable",
            Self::ClientError(ClientError::ProxyAuthenticationRequired) => {
                b"407 ProxyAuthenticationRequired"
            }

            Self::ClientError(ClientError::RequestTimeout) => b"408 RequestTimeout",
            Self::ClientError(ClientError::Conflict) => b"409 Conflict",
            Self::ClientError(ClientError::Gone) => b"410 Gone",
            Self::ClientError(ClientError::LengthRequired) => b"411 LengthRequired",
            Self::ClientError(ClientError::PreconditionFailed) => b"412 PreconditionFailed",
            Self::ClientError(ClientError::ContentTooLarge) => b"413 ContentTooLarge",
            Self::ClientError(ClientError::URITooLong) => b"414 URITooLong",
            Self::ClientError(ClientError::UnsupportedMediaType) => b"415 UnsupportedMediaType",
            Self::ClientError(ClientError::RangeNotSatisfiable) => b"416 RangeNotSatisfiable",
            Self::ClientError(ClientError::ExpectationFailed) => b"417 ExpectationFailed",
            Self::ClientError(ClientError::Imateapot) => b"418 Imateapot",
            Self::ClientError(ClientError::MisdirectedRequest) => b"421 MisdirectedRequest",
            Self::ClientError(ClientError::UnprocessableContent) => b"422 UnprocessableContent",
            Self::ClientError(ClientError::Locked) => b"423 Locked",
            Self::ClientError(ClientError::FailedDependency) => b"424 FailedDependency",
            Self::ClientError(ClientError::TooEarly) => b"425 TooEarly",
            Self::ClientError(ClientError::UpgradeRequired) => b"426 UpgradeRequired",
            Self::ClientError(ClientError::PreconditionRequired) => b"428 PreconditionRequired",
            Self::ClientError(ClientError::TooManyRequests) => b"429 TooManyRequests",
            Self::ClientError(ClientError::RequestHeaderFieldsTooLarge) => {
                b"431 RequestHeaderFieldsTooLarge"
            }

            Self::ClientError(ClientError::UnavailableForLegalReasons) => {
                b"451 UnavailableForLegalReasons"
            }

            Self::ServerError(ServerError::InternalServerError) => b"500 InternalServerError",
            Self::ServerError(ServerError::NotImplemented) => b"501 NotImplemented",
            Self::ServerError(ServerError::BadGateway) => b"502 BadGateway",
            Self::ServerError(ServerError::ProcessUnavailable) => b"503 ProcessUnavailable",
            Self::ServerError(ServerError::GatewayTimeout) => b"504 GatewayTimeout",
            Self::ServerError(ServerError::HTTPVersionNotSupported) => {
                b"505 HTTPVersionNotSupported"
            }

            Self::ServerError(ServerError::VariantAlsoNegotiates) => b"506 VariantAlsoNegotiates",
            Self::ServerError(ServerError::InsufficientStorage) => b"507 InsufficientStorage",
            Self::ServerError(ServerError::LoopDetected) => b"508 LoopDetected",
            Self::ServerError(ServerError::NotExtended) => b"510 NotExtended",
            Self::ServerError(ServerError::NetworkAuthenticationRequired) => {
                b"511 NetworkAuthenticationRequired"
            }
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

// enum Status {
//     Reject(ErrorStatus),
//     Accept(AcceptStatus),
// }

/// error response status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorStatus {
    Client(ClientError),
    Server(ServerError),
}

impl ErrorStatus {
    pub fn text(&self) -> &str {
        match self {
            Self::Client(ce) => ce.text(),
            Self::Server(se) => se.text(),
        }
    }

    pub fn code(&self) -> u16 {
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

// impl ToTokens for ErrorStatus {
//     fn to_tokens(&self, tokens: &mut TS2) {
//         tokens.append(<ErrorStatus as Into<TokenTree>>::into(*self))
//     }
// }
//
// impl From<ErrorStatus> for TokenTree {
//     fn from(err: ErrorStatus) -> Self {
//         let [var, subtype, subvar] = {
//             let s = err.to_string();
//             let mut iter = s
//                 .split("::")
//                 .map(|s| Ident::new(s.trim(), Span::call_site()));
//
//             [
//                 iter.next().unwrap(),
//                 iter.next().unwrap(),
//                 iter.next().unwrap(),
//             ]
//         };
//
//         Group::new(
//             Delimiter::None,
//             quote::quote! { ErrorStatus::#var(pheasant:: #subtype::#subvar) },
//         )
//         .into()
//     }
// }

impl fmt::Display for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.str_var(), self.str_lit())
    }
}
