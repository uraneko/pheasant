#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Scheme {
    // Git,
    // Blob,
    // Data,
    // Javascript,
    // Urn,
    Http,
    Https,
    // Ws,
    // Wss,
    // File,
    // Ftp,
    // Resource,
    // Ssh
    // Custom(String),
}

impl Scheme {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            // Self::Ws => "ws",
            // Self::Wss => "wss",
            // Self::File => "file",
            // Self::Ftp => "ftp",
            // Self::Blob => "blob",
            // Self::Data => "data",
            // Self::Javascript => "javascript",
            // Self::Urn => "urn",
            // Self::Resource => "resource",
            // Self::Git => "git",
            // Self::Ssh => "ssh",
        }
    }

    pub fn default_port(&self) -> u16 {
        match self {
            Self::Http => 80,
            Self::Https => 443,
            // Self::Ssh => 22,
            // Self::Git => 9418,
            // Self::Ws => 80,
            // Self::Wss => 443,
            // Self::File => 0,
            // Self::Ftp => 21,
        }
    }
}

impl std::str::FromStr for Scheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_uppercase().as_str() {
            "HTTP" => Ok(Self::Http),
            "HTTPS" => Ok(Self::Https),
            // "WS" => Ok(Self::Ws),
            // "WSS" => Ok(Self::Wss),
            // "FILE" => Ok(Self::File),
            // "FTP" => Ok(Self::Ftp),
            _ => Err(()),
        }
    }
}
