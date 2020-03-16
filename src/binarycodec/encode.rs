use crate::protocol::{Clienthandshake};

pub trait Encode {
    fn encode(&self) -> &[u8];
}

impl Encode for Clienthandshake {
    fn encode(&self) -> &[u8] {
        self.as_ref()
    }
}
