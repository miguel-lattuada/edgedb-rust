use crate::protocol::{Serverhandshake, ErrorResponse};

pub enum ServerMessage {
    Serverhandshake(Serverhandshake),
    ErrorResponse(ErrorResponse)
}

impl ServerMessage {
    pub fn create(bytes: &[u8]) -> Result<ServerMessage, ()> {
        match bytes[0] {
            0x78 => {
                let sh = Serverhandshake::new();
                Ok(sh).map(ServerMessage::Serverhandshake)
            }
            _ => {
                let er = ErrorResponse::new();
                Ok(er).map(ServerMessage::ErrorResponse)
            }
        }
    }
}