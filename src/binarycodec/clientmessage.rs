use crate::protocol::Clienthandshake;

pub enum ClientMessage {
    Clienthandshake(Clienthandshake)
}

impl ClientMessage { 
    fn encode(&self) -> &[u8] {
        match self {
            ClientMessage::Clienthandshake(ch) => {
                ch::encode()
            }
        }
    }
}