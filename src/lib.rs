//! The mailslot api is a Single Consumer Multiple Sender IPC system
//! natively supported by windows. A single server can listen to a
//! mail slot and any number of clients. Below is a naive example
//! that counts to 10, printing the numbers to the terminal.
//!
//! ```rust
//! use mail_slot::{MailslotName, MailslotServer, MailslotClient};
//! let name = MailslotName::local("naive");
//! let mut server = MailslotServer::new(&name).unwrap();
//! let mut client = MailslotClient::new(&name).unwrap();
//! for i in 0..10 {
//!     client.send_message(i.to_string().as_bytes()).unwrap();
//! }
//! while let Some(msg) = server.get_next_unread().unwrap() {
//!     let msg_str = String::from_utf8(msg).unwrap();
//!     println!("message from client {}", msg_str);
//! }
//! ```
use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

mod client;
mod error;
mod server;
pub use client::MailslotClient;
pub use error::Error;
pub use server::MailslotServer;

/// A mailslot name has a specific
/// format, this struct will ensure
/// the correct format is used
///
/// The format will always
/// start with two slashes,
/// then the domain name
/// another slash the word "mailslot"
/// another slash and and any valid
/// windows path for the remainder
///
///
/// ```rust
/// # use mail_slot::MailslotName;
/// let local_path = MailslotName::local("name");
/// assert_eq!(r"\\.\mailslot\name", local_path.to_string());
///
/// let net_path = MailslotName::network("domain", "name");
/// assert_eq!(r"\\domain\mailslot\name", net_path.to_string());
/// ```
pub struct MailslotName {
    pub domain: String,
    pub path: String,
}
impl MailslotName {
    /// This will create a mailslot name
    /// with a local domain
    pub fn local(path: &str) -> Self {
        Self {
            domain: ".".to_string(),
            path: path.to_string(),
        }
    }
    /// This will create a mailslot name
    /// with a domain and path
    pub fn network(domain: &str, path: &str) -> Self {
        Self {
            domain: domain.to_string(),
            path: path.to_string(),
        }
    }
    /// This will create a mailslot name
    /// with a default domain
    pub fn default_domain(path: &str) -> Self {
        Self {
            domain: "*".to_string(),
            path: path.to_string(),
        }
    }
}

impl ToString for MailslotName {
    fn to_string(&self) -> String {
        format!(r"\\{}\mailslot\{}", self.domain, self.path)
    }
}

fn to_win_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MailslotServer;
    #[test]
    fn round_trip() {
        let messages: Vec<&[u8]> = vec![b"one", b"two", b"three"];
        let name = MailslotName::local("client_send");
        let mut server = MailslotServer::new(&name).unwrap();
        let mut client = MailslotClient::new(&name).unwrap();
        for msg in &messages {
            client.send_message(msg).unwrap();
        }
        let mut rec = vec![];
        while let Some(msg) = server.get_next_unread().unwrap() {
            rec.push(msg);
        }
        assert_eq!(messages[0], rec[0].as_slice());
        assert_eq!(messages[1], rec[1].as_slice());
        assert_eq!(messages[2], rec[2].as_slice());
    }
}
