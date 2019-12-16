#[derive(Debug)]
pub enum Error {
    MessageTooLarge(usize),
    Io(std::io::Error),
    Unknown(String),
    Other(Box<dyn std::error::Error>),
}
impl Error {
    pub fn last_os_error() -> Self {
        std::io::Error::last_os_error().into()
    }
    #[cfg(target_os = "macos")]
    pub fn macos_error(code: i32) -> Self {
        Self::Unknown(crate::mac::error_name(code))
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MessageTooLarge(size) => write!(f, "Message with length {} is too large, messages sent over a network cannot be larger than 424 bytes", size),
            Self::Io(e) => write!(f, "IO Error: {}", e),
            Self::Unknown(msg) => write!(f, "{}", msg),
            Self::Other(e) => write!(f, "Unknown Error: {}", e),
        }
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Self::Io(other)
    }
}
