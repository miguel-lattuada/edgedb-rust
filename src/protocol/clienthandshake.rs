// struct ClientHandshake {
//     // Message type ('V')
//     int8        mtype = 0x56;

//     // Length of message contents in bytes,
//     // including self.
//     int32       message_length;

//     // Requested protocol major version.
//     int16       major_ver;

//     // Requested protocol minor version.
//     int16       minor_ver;

//     // Number of connection parameters.
//     int16       num_params;

//     // Connection parameters.
//     Param       params[num_params];

//     // Number of protocol extensions.
//     int16       num_exts;

//     // Requested protocol extensions.
//     ProtocolExt exts[num_exts];
// };

// struct Param {
//     string parameter_name;
//     string parameter_value;
// };

// struct ProtocolExt {
//     // Extension name.
//     string  extname;
//     // Extension headers.
//     Headers extheaders;
// };

use crate::protocol::{Sequence};

pub struct Clienthandshake {
    params: std::collections::HashMap<String, String>,
    message: Sequence
}

impl Clienthandshake {
    pub fn new(params: std::collections::HashMap<String, String>) -> Clienthandshake {
        Clienthandshake {
            params: params,
            message: Sequence::new()
        }
    }

    pub fn setup_message(&mut self) {
        self.set_message_type();
        self.set_message_versioning();
        self.set_message_params_number();
        self.set_message_params();
        self.set_message_ext_number(0);
        self.set_message_length();
    }

    // #region Set message data (helpers)
    fn set_message_type(&mut self) {
        self.message.push_char('V');
    }

    fn set_message_versioning(&mut self) {
        self.message.push_u16(1);
        self.message.push_u16(0);
    }

    fn set_message_params_number(&mut self) {
        self.message.push_u16(self.params.len() as u16);
    }

    fn set_message_params(&mut self) {
        loop {
            let (param_name, param_value) = self.params.iter().next().unwrap();
            self.push_parameter(param_name.to_string(), param_value.to_string());
        }
    }
    
    fn set_message_ext_number(&mut self, n: u16) {
        self.message.push_u16(n);
    }
    
    fn set_message_length(&mut self) {
        self.message.insert_u32(self.message.length as u32, 1);
    }
    // #endregion

    /**
     * Push a parameter into the message
     */
    fn push_parameter(&mut self, name: String, value: String) {
        for c in name.chars() {
            self.message.push_char(c);
        }
        for c in value.chars() {
            self.message.push_char(c);
        }
    }

    /**
     * Return message as a reference
     */
    pub fn as_ref(&mut self) -> &[u8] {
        self.message.as_ref()
    }
}