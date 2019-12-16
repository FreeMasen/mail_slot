use tokio;
use mail_slot::*;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = MailslotName::local("async");
    let mut server = MailslotServer::new(&name)?;
    let client = MailslotClient::new(&name)?;
    ::std::thread::spawn(move || {
        let mut client = client;
        println!("Waiting for half a sec");
        std::thread::sleep(std::time::Duration::from_millis(500));    
        for i in 0..10 {
            println!("sending msg {}", i);
            client.send_message(i.to_string().as_bytes()).unwrap();
            println!("waiting for half a sec");
            std::thread::sleep(std::time::Duration::from_millis(500));    
        }
    });
    for _ in 0u8..10 {
        let msg = server.get_next_unread_async().await?;
        let msg_str = String::from_utf8(msg)?;
        println!("message from client {}", msg_str);
    }
    Ok(())
}