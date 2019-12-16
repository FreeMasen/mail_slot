use mail_slot::{MailslotName, MailslotServer, MailslotClient};


pub fn main() {
    let name = MailslotName::local("naive");
    let mut server = MailslotServer::new(&name).unwrap();
    let mut client = MailslotClient::new(&name).unwrap();
    for i in 0..10 {
        client.send_message(i.to_string().as_bytes()).unwrap();
        while let Some(msg) = server.get_next_unread().unwrap() {
            let msg_str = String::from_utf8(msg).unwrap();
            println!("message from client {}", msg_str);
        }
    }
}