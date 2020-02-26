use std::net::TcpStream;

use crate::connection::ConnectionOptions;
use crate::binarycodec::{ServerMessage, ClientMessage};

pub struct Connection {
    options: ConnectionOptions,
    stream: Option<TcpStream>
}

impl Connection {
    pub fn new(options: ConnectionOptions) -> Connection {
        Connection {
            options: options,
            stream: None
        }
    }

    /**
     * Open connection
     */
    pub fn open(&mut self) -> &mut Self { 
        let tcp_conn = TcpStream::connect(&self.options.host);

        match tcp_conn {
            Ok(stream) => self.stream = Some(stream),
            Err(_) => self.stream = None
        }

        self
    }

    /**
     * Send client message through stream connection
     */
    pub fn write(message: ClientMessage) -> Result<ServerMessage, ()> {
        Err(())
    }

    /**
     * Read server message from stream connection
     */
    fn read(bytes: &[u8]) -> Result<ServerMessage, ()> {
        ServerMessage::create(bytes)
    }
}

/**
 * 
 * /
 * let con = Connection::new(ConnectionOptions { host: "myhost.edgedb".to_string() });
 * 
 * let client_handshake = Clienthandshake::new(HashMap::new());
 * 
 * con.write(client_handshake);
 * 
 * let response = con.read()
 * 
 * match response {
 *  ErrorResponse => panic!();
 * }
 */
pub fn nddr() { }