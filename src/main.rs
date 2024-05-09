use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io::Write;
use std::io::BufReader;
use serde_json::{json, Value};

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

include!("handler.rs");

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    id: String,
    method: String,
    #[serde(default)] // use default value [], in case of lack of value
    params: Vec<MessageData>
}

#[derive(Deserialize, Serialize, Debug)]
struct MessageData {
    data: HashMap<String, Value>
}



fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}



fn handle_client(stream: TcpStream) {
    let mut buffered_stream = BufReader::new(&stream);
    let mut line = String::new();

    buffered_stream.read_line(&mut line).unwrap();
    print!("Message received: {}", line);

    let msg: Message = serde_json::from_str(&line).unwrap();


    let resp_data = handler(msg.params[0].data.clone());

    let resp =  json!({
        "id": msg.id,
        "result": json!({
            "data":  resp_data
        })
    });
    let response = resp.to_string();
    (&stream).write_all(response.as_bytes()).unwrap();
    (&stream).write_all(b"\n").unwrap();
}