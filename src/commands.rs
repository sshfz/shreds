use std::collections::HashMap;
use crate::store::{self, *};
pub enum Command {
   PING,
   SET,
   GET,
   DELETE,
   NULL
}

pub fn shredisDB(map: HashMap<String, String>) -> HashMap<String, String> {
    return map;
}

pub async fn parse_command(input: &str) -> (Command, String) {
    let trimmed = input.trim();
    let store = Store::getStore(); 
    // Handle RESP array format: *1\r\n$4\r\nPING\r\n
    if trimmed.starts_with('*') {
        // Split by \r\n to parse RESP protocol
        let parts: Vec<&str> = trimmed.split("\r\n").collect();
        // let mut command=String::from("");
        let mut parsedRedis: Vec<String> = vec![];
        // Find the actual command (it follows the bulk string length indicator)
        for i in 0..parts.len() {
            if parts[i].starts_with('$') && i + 1 < parts.len() {
                let command = parts[i + 1].to_string();
                parsedRedis.push(command);
               
            }
        }

        println!("{:?}", parsedRedis);

        match parsedRedis[0].as_str() {
            "PING" => Command::PING,
            "SET" => {
                let key = match parsedRedis.get(1).filter(|s| !s.is_empty()) {
                    Some(k) => k,
                    None => return (Command::SET, String::from("key is required"))
                };
                let value: &String = match parsedRedis.get(2).filter(|s| !s.is_empty()) {
                    Some(k) => k,
                    None => return (Command::SET, String::from("value is required"))
                };
                store.set(key.to_string(), value.to_string()).await;
                return (Command::SET, String::from("value saved"));
            },
            "GET" => {
                let key = &parsedRedis[1];
                let val = store.get(key.to_string()).await;
                return match val {
                    Some(value) => (Command::GET, value), 
                    None => (Command::GET, String::from("NO_KEY_FOUND")),
                }
            },
            "DELETE" => {
                let key = &parsedRedis[1];
                store.delete(key.to_string()).await;
                return (Command::DELETE, String::from("deleted"));
            },
            _ => Command::NULL
        };
    }
    
    let upper = trimmed.to_uppercase();
    match upper.as_str() {
        "PING" => (Command::PING, String::from("")),
        _ => (Command::NULL, String::from("")),
    }
}

pub fn setAction() {
    println!("saving value to db...")
}



pub fn parse_inline_commands(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() {
       return Command::NULL
    }
    
    println!("{}eeeee", parts[0].to_uppercase().as_str());
    match parts[0].to_uppercase().as_str() {
        "PING" => Command::PING,

        _ => Command::NULL,
    }
}

pub fn try_parse_resp_command(buffer: &mut Vec<u8>) -> Command {
    let input = std::str::from_utf8(buffer).unwrap();
    if !input.starts_with("*1\r\n$4\r\n") {
        return Command::NULL;
    }
    println!("{}sssss", input);
    if let Some(pos) = input.find("\r\nPING\r\n") {
        // Consume parsed bytes
        let consumed = pos + "\r\nPING\r\n".len();
        buffer.drain(..consumed);
        return Command::PING;
    }

    Command::NULL // Not enough data 
}