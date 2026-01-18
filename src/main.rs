use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

use crate::{commands::{parse_command, parse_inline_commands, try_parse_resp_command}, store::Store};
mod commands;
mod store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener: TcpListener= TcpListener::bind("127.0.0.1:6379").await?;
    println!("server listening on 127.0.0.1:6379");
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Client connected : {}", addr);

        tokio::spawn(async move {
            println!("Sending welcome message to {}", addr);
            // let welcome = b"+PONG\n";

            // if let Err(e) = socket.write_all(welcome).await {
            //    eprintln!("Failed to send welcome to {}: {}", addr, e);
            //    return;
            // }
            // println!("Welcome message sent to {}", addr);

            let mut buf: [u8; 1024] = [0u8; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client {} disconnected", addr);
                        break;
                    }
                    Ok(n) => {
                        let received = String::from_utf8_lossy(&buf[..n]);
                        // let mut buffer: Vec<u8> = Vec::new();
                        // buffer.extend_from_slice(&buf[..n]);

                        // if is_resp(&buffer) {
                            match parse_command(&received).await {
                               (commands::Command::PING, value) => {
                                 let res =  b"+PONG\r\n";
                                 println!("Sending bytes: {:?}", res);
                                 if let Err(e) = socket.write_all(res).await {
                                  eprintln!("write error: {}", e)
                                } 
                              }
                               (commands::Command::SET, val)=> {
                                 let res = format!("+{}\r\n", val);
                                 if let Err(e) = socket.write_all(res.as_bytes()).await {
                                    eprintln!("write error: {}", e)
                                 }
                               }
                               (commands::Command::GET, val) => {
                                 let res = format!("+{}\r\n", val);
                                 if let Err(e) = socket.write_all(res.as_bytes()).await {
                                    eprintln!("write error: {}", e)
                                 }
                               }
                               (commands::Command::DELETE, val) => {
                                 let res = b"+VALUE_DELETED_FROM_DB\r\n";
                                 if let Err(e) = socket.write_all(res).await {
                                    eprintln!("write error: {}", e);
                                 }
                               }
                               (commands::Command::NULL, val) => {
                                 let response: &[u8; 22] = b"-ERR unknown command\r\n";
                                 let _ = socket.write_all(response).await;
                             }
                          }
                    }
                     Err(e) => {
                        eprintln!("Error reading from {}: {}", addr, e);
                        break;
                    }
                }
            }
        });
    }
}

fn is_resp(buffer: &Vec<u8>) -> bool {
    buffer.first() == Some(&b'*')
}
