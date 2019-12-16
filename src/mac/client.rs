use crate::Error;
use mach::{
    bootstrap::{bootstrap_look_up, bootstrap_port},
    port::{mach_port_t, MACH_PORT_NULL},
    message::{
        mach_msg_header_t,
        MACH_MSG_TYPE_COPY_SEND,
        MACH_MSGH_BITS,
        mach_msg,
        MACH_SEND_MSG,
        MACH_MSG_TIMEOUT_NONE,
    },
    kern_return::KERN_SUCCESS
};

pub struct MailslotClient {
    port: mach_port_t,
}

impl MailslotClient {
    pub fn new(name: &super::MailslotName) -> Result<Self, Error> {
        let port = create_port(&name.to_string())?;
        Ok(Self {
            port,
        })
    }
    pub fn send_message(&mut self, msg: &[u8]) -> Result<(), Error> {
        send_message(self.port, msg)
    }
}

pub fn create_port(name: &str) -> Result<mach_port_t, crate::Error> {
    let mut ret = 0;
    let res = unsafe {
        bootstrap_look_up(bootstrap_port, name.as_ptr() as _, &mut ret)
    };
    if res != KERN_SUCCESS {
        return Err(Error::macos_error(res));
    }
    Ok(ret)
}

pub fn send_message(port: mach_port_t, msg: &[u8]) -> Result<(), crate::Error> {
    let mut header = mach_msg_header_t::default();
    header.msgh_bits = MACH_MSGH_BITS(MACH_MSG_TYPE_COPY_SEND, 0);
    header.msgh_remote_port = port;
    header.msgh_local_port = MACH_PORT_NULL;
    let mut message = Message {
        header,
        contents: msg,
    };
    let res = unsafe {
        mach_msg(
            &mut message.header as _,
            MACH_SEND_MSG,
            std::mem::size_of::<Message>() as _,
            0,
            MACH_PORT_NULL,
            MACH_MSG_TIMEOUT_NONE,
            MACH_PORT_NULL,
        )
    };
    if res != KERN_SUCCESS {
        return Err(Error::macos_error(res));
    }
    Ok(())
}

#[repr(C)]
struct Message<'a> {
    header: mach_msg_header_t,
    contents: &'a [u8],
}