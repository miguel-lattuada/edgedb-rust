mod protocol;

use std::io::prelude::*;
use std::net::{TcpStream};
use std::collections::{HashMap};
use protocol::{Clienthandshake, Serverhandshake};

fn main() {
    // Create client message
    let mut ch_message = Clienthandshake::new(HashMap::new());
    ch_message.setup_message();

    // Create server message
    let mut sh = Serverhandshake::new();

    // Create connection
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let write_result = stream.write(ch_message.as_ref());
    let read_result = stream.read_to_end(sh.read_into());

    // TCP Connection related
    println!("Bytes written {}", write_result.unwrap());
    println!("Bytes read {}", read_result.unwrap());

    // Edge server related
    println!("Message type: {}", sh.get_message_type());
    println!("Message length {}", sh.get_message_length());
    println!("Message versioning {:?}", sh.get_message_versioning());
}