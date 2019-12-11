use mail_slot::{MailslotName, MailslotServer};


pub fn main() {
    let name = MailslotName::local("just_listening");
    let mut server = MailslotServer::new(&name).unwrap();
    println!("listening for message on {}", name.to_string());
    loop {
        match server.get_next_unread() {
            Ok(maybe_msg) => {
                if let Some(msg) = maybe_msg {
                    println!("> {}", String::from_utf8(msg).unwrap_or(String::from("bad utf8 in msg")));
                }
            }
            Err(e) => {
                eprintln!("error getting next msg: {}", e);
            }
        }
    }
}