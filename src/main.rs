use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};
use std::str::from_utf8;
use std::time::SystemTime;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TimestampMessage {
    body: String,
    sent: u128,
    recieved: u128,
    returned: u128
}


fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").expect("Could not bind!");

    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("Failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 15000];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        println!("Reading bytes, length, {}", bytes_read);
        if bytes_read == 0 { return Ok(()) }

        let response_text = from_utf8(&buf[..bytes_read]).unwrap();

        let mut msg_obj: TimestampMessage = serde_json::from_str(&response_text).unwrap();

        msg_obj.recieved = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

        let text_to_send = serde_json::to_string(&msg_obj).unwrap();

        let bytes_to_send = text_to_send.as_bytes();

        stream.write(&bytes_to_send)?;
    }
}