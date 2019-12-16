use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub struct MailslotClient {
    file: File,
    has_domain: bool,
}

use super::{Error, MailslotName};

impl MailslotClient {
    pub fn new(name: &MailslotName) -> Result<Self, Error> {
        let has_domain = name.domain == ".";
        let file = OpenOptions::new().write(true).open(&name.to_string())?;
        Ok(Self { file, has_domain })
    }
    pub fn send_message(&mut self, msg: &[u8]) -> Result<(), Error> {
        if self.has_domain && msg.len() > 424 {
            return Err(Error::MessageTooLarge(msg.len()));
        }
        self.file.write_all(msg)?;
        Ok(())
    }
}
