
use crate::{windows::to_win_string, Error, MailslotName};
use std::{
    fs::File,
    io::Read,
    os::windows::io::{AsRawHandle, FromRawHandle},
};
use winapi::um::{
    handleapi::INVALID_HANDLE_VALUE,
    winbase::{CreateMailslotW, GetMailslotInfo},
    winnt::{MAILSLOT_NO_MESSAGE, MAILSLOT_WAIT_FOREVER},
};

pub struct MailslotServer {
    handle: File,
}

impl MailslotServer {
    pub fn new(name: &MailslotName) -> Result<Self, Error> {
        Ok(Self {
            handle: make_mail_slot(&name.to_string())?,
        })
    }

    pub fn get_next_unread(&mut self) -> Result<Option<Vec<u8>>, Error> {
        if let Some(next_len) = self.get_next_length()? {
            let mut buf = vec![0; next_len as _];
            self.handle
                .read_exact(&mut buf)
                .expect("failed to read file");
            Ok(Some(buf))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn get_next_length(&self) -> Result<Option<u32>, Error> {
        let mut msg_size = 0;
        let mut msg_count = 0;
        let info = unsafe {
            GetMailslotInfo(
                self.handle.as_raw_handle() as _,
                0 as _,
                &mut msg_size,
                &mut msg_count,
                0 as _,
            )
        };
        if info == 0 {
            Err(Error::last_os_error())
        } else if msg_size == MAILSLOT_NO_MESSAGE || msg_count == 0 {
            Ok(None)
        } else {
            Ok(Some(msg_size))
        }
    }
}

pub fn make_mail_slot(name: &str) -> Result<File, Error> {
    let name = to_win_string(name);
    unsafe {
        let slot = CreateMailslotW(name.as_ptr() as _, 0, MAILSLOT_WAIT_FOREVER, 0 as _);
        if slot == INVALID_HANDLE_VALUE {
            Err(Error::Io(::std::io::Error::last_os_error()))
        } else {
            // Since we create this file handle in this scope
            // no one else could be holding this file handle
            // subsequent calls to CreateMailslotW with the same name
            // will error, maintaining the uniqueness of the file
            Ok(std::fs::File::from_raw_handle(slot as _))
        }
    }
}
