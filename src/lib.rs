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
//!     while let Some(msg) = server.get_next_unread().unwrap() {
//!         let msg_str = String::from_utf8(msg).unwrap();
//!         println!("message from client {}", msg_str);
//!     }
//! }
//! ```


#[cfg(windows)]
mod windows;
#[cfg(target_os = "macos")]
mod mac;

mod error;

#[cfg(windows)]
pub use windows::{
    client::MailslotClient,
    server::MailslotServer,
    MailslotName,
};

#[cfg(target_os = "macos")]
pub use mac::{
    client::MailslotClient,
    server::MailslotServer,
    MailslotName,
};
pub use error::Error;





