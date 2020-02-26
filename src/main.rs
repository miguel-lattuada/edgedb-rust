mod binarycodec;
mod connection;
mod protocol;

use std::io::prelude::*;
use std::net::{TcpStream};
use std::collections::{HashMap};
use protocol::{Clienthandshake, Serverhandshake, ErrorResponse};

fn main() {
    // Create client message
    let mut ch_message = Clienthandshake::new(HashMap::new());
    ch_message.setup_message();

    // Create server message
    // let mut sh = Serverhandshake::new();

    let mut em = ErrorResponse::new();

    // Create connection
    // let mut stream = TcpStream::connect("165.227.3.104:5656").unwrap();

    if let Ok(mut stream) = TcpStream::connect("165.227.3.104:5656") {
        let write_result = stream.write(ch_message.as_ref());
        let read_result = stream.read_to_end(em.read_into());
    
        // TCP Connection related
        println!("Bytes written {}", write_result.unwrap());
        println!("Bytes read {}", read_result.unwrap());
    
        // Edge server related
        println!("Message type: {}", em.get_message_type());
        println!("Message length {}", em.get_message_length());
        println!("Message error severity {}", em.get_error_severity());
        println!("Message error code {}", em.get_error_code());
        println!("Message error message size {}", em.get_error_message_size());
        println!("Message error message {}", em.get_error_message());
    } else {
        println!("Could not connect to server");
    }
}