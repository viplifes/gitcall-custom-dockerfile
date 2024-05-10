use std::env;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io::Write;
use std::io::BufReader;
use std::io::Error;
use serde_json::{json, Value};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

include!("handler.rs");

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    id: Id,
    method: String,
    #[serde(default)] // use default value [], in case of lack of value
    params: Vec<MessageData>
}

#[derive(Deserialize, Serialize, Debug)]
struct MessageData {
    data: HashMap<String, Value>
}

/// Request Id
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id {
	/// No id (notification)
	Null,
	/// Numeric id
	Num(u64),
	/// String id
	Str(String),
}



fn main() -> Result<(), Error> {

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

    thread::spawn(move || {
        while !term.load(Ordering::Relaxed) {
            // Do some time-limited stuff here
            // (if this could block forever, then there's no guarantee the signal will have any
            // effect).
            std::process::exit(0)
        }
    });

    let addr = env::var("USERCODE_PROXY_ADDR").expect("$USERCODE_PROXY_ADDR is not set");
    let listener = TcpListener::bind(addr).unwrap();
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

    Ok(())
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