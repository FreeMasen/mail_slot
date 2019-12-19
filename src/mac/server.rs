use core::task::Poll;
use core::task::Context;
use crate::Error;
use mach::{
    bootstrap::{bootstrap_register, bootstrap_port},
    port::{mach_port_t, MACH_PORT_RIGHT_RECEIVE, MACH_PORT_NULL, MACH_MSG_TIMED_OUT},
    kern_return::KERN_SUCCESS,
    mach_port::mach_port_allocate,
    traps::mach_task_self,
    message::{
        mach_msg_header_t,
        mach_msg_trailer_t,
        MACH_RCV_MSG,
        MACH_RCV_TIMEOUT,
        mach_msg,
    },
};

pub struct MailslotServer {
    port: mach_port_t,
}

impl MailslotServer {
    pub fn new(name: &super::MailslotName) -> Result<Self, Error> {
        let port = create_port(&name.to_string())?;
        Ok(MailslotServer {
            port,
        })
    }
    pub fn get_next_unread(&mut self) -> Result<Option<Vec<u8>>, Error> {
        let ret = get_next_message(self.port)?;
        Ok(ret)
    }
    pub async fn get_next_unread_async(&mut self) -> Result<Vec<u8>, Error> {
        FutureMessage {
            port: self.port
        }.await
    }
}


pub struct FutureMessage {
    port: mach_port_t,
}

impl std::future::Future for FutureMessage {
    type Output = Result<Vec<u8>, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("polling");
        match dbg!(get_next_message(self.port)) {
            Ok(Some(message)) => {
                Poll::Ready(Ok(message))
            },
            Ok(None) => {
                Poll::Pending
            },
            Err(e) => {
                Poll::Ready(Err(e))
            }
        }
    }
    
}

pub fn create_port(name: &str) -> Result<mach_port_t, Error> {
    let mut ret = 0;
    let res = unsafe {
        mach_port_allocate(mach_task_self(), MACH_PORT_RIGHT_RECEIVE, &mut ret as _)
    };
    if res != KERN_SUCCESS {
        return Err(Error::macos_error(res));
    }
    let res = unsafe {
        bootstrap_register(bootstrap_port, name.as_ptr() as _, ret as _)
    };
    if res != KERN_SUCCESS {
        return Err(Error::macos_error(res));
    }
    Ok(ret)
}

fn get_next_message(port: mach_port_t) -> Result<Option<Vec<u8>>, Error> {
    let header = Default::default();
    let trailer = Default::default();
    let mut msg =  Message {
        header,
        content: &[],
        trailer,
    };
    let res = unsafe {
        mach_msg(
            &mut msg.header as _,
            MACH_RCV_MSG | MACH_RCV_TIMEOUT,
            0,
            std::mem::size_of::<Message>() as _,
            port,
            100,
            MACH_PORT_NULL,
        )
    };
    if res == MACH_MSG_TIMED_OUT {
        return Ok(None)
    }
    if res != KERN_SUCCESS {
        return Err(Error::macos_error(res));
    }
    Ok(Some(msg.content.to_vec()))
}

#[repr(C)]
struct Message<'a> {
    header: mach_msg_header_t,
    content: &'a [u8],
    trailer: mach_msg_trailer_t,
}