
pub mod client;
pub mod server;


pub struct MailslotName {
    pub domain: String,
    pub path: String,
}

impl MailslotName {
    /// This will create a mailslot name
    /// with a local domain
    pub fn local(path: &str) -> Self {
        Self {
            domain: "localhost".to_string(),
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
            domain: "example".to_string(),
            path: path.to_string(),
        }
    }
}

impl ToString for MailslotName {
    fn to_string(&self) -> String {
        format!(r"org.{}.{}", self.path, self.domain)
    }
}

pub(crate) fn error_name(code: i32) -> String {
    use mach::bootstrap::bootstrap_strerror;
    let mut idx = unsafe {
        bootstrap_strerror(code) as usize
    };
    let mut ret = String::new();
    unsafe {
        loop {
            let c: u8 = std::ptr::read(idx as _);
            if c == 0 {
                break;
            }
            ret.push(c as char);
            idx += 1;
        }
    }
    ret

}

#[cfg(test)]
mod test {
    use super::client::MailslotClient;
    use super::server::MailslotServer;
    use super::MailslotName;
    #[test]
    fn round_trip() {
        let messages: Vec<&[u8]> = vec![b"one", b"two", b"three"];
        let name = MailslotName::network("example", "round_trip");
        let mut server = MailslotServer::new(&name).unwrap();
        let mut client = MailslotClient::new(&name).unwrap();
        for msg in &messages {
            client.send_message(msg).unwrap();
        }
        let mut rec = vec![];
        while let Some(msg) = server.get_next_unread().unwrap() {
            rec.push(dbg!(msg));
        }
        assert_eq!(messages[0], rec[0].as_slice());
        assert_eq!(messages[1], rec[1].as_slice());
        assert_eq!(messages[2], rec[2].as_slice());
    }
}