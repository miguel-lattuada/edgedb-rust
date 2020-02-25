// struct ServerHandshakeMessage {
//     // Message type ('v')
//     int8        mtype = 0x76;

//     // Length of message contents in bytes,
//     // including self.
//     int32       message_length;

//     // maximum supported or client-requested
//     // protocol major version, whichever is greater.
//     int16       major_ver;

//     // maximum supported or client-requests
//     // protocol minor version, whichever
//     // is greater.
//     int16       minor_ver;

//     // number of supported protocol extensions
//     int16       num_exts;

//     // supported protocol extensions
//     ProtocolExt exts[num_exts];
// };

// struct ProtocolExt {
//     // extension name
//     string  extname;

//     // extension headers
//     Headers extheaders;
// };

// Pseudo-bytes representation of structure
// [v, l, l, l, l, mjv, mjv, miv, miv, nexts, nexts, ...exts]

use crate::protocol::{Sequence};

pub struct Serverhandshake {
    message: Sequence
}

impl Serverhandshake {
    pub fn new() -> Serverhandshake {
        Serverhandshake {
            message: Sequence::new()
        }
    }

    /**
     * Read stream into message sequence
     */
    pub fn read_into(&mut self) -> &mut Vec<u8> {
        self.message.as_mut()
    }

    /**
     * Get message type as a character ('v')
     */
    pub fn get_message_type(&mut self) -> char {
        self.message.read_u8(0) as char
    }

    /**
     * Get entire message length
     */
    pub fn get_message_length(&mut self) -> u32 {
        self.message.read_u32(1)
    }

    /**
     * Return a tuple (major version, minor version) represented as u16 both
     */
    pub fn get_message_versioning(&mut self) -> (u16, u16) {
        (self.message.read_u16(5), self.message.read_u16(7))
    }
}