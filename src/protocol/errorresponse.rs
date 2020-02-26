// struct ErrorResponse {
//     // Message type ('E')
//     int8                mtype = 0x45;

//     // Length of message contents in bytes,
//     // including self.
//     int32               message_length;

//     // Error severity.
//     int8<ErrorSeverity> severity;

//     // Error code.
//     int32               code;

//     // Error message
//     string              message;

//     // Other error attributes.
//     Headers             attributes;
// };

// enum ErrorSeverity {
//     ERROR = 120,
//     FATAL = 200,
//     PANIC = 255
// };


// [t, l, l, l, l, s, c, c, c, c, sz, sz, sz, sz, ..sz, ..h]
use crate::binarycodec::Sequence;

pub struct ErrorResponse {
    message: Sequence
}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ErrorResponse {
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
     * Get error severity
     */
    pub fn get_error_severity(&mut self) -> u8 {
        self.message.read_u8(5)
    }

    /**
     * Get error code
     */
    pub fn get_error_code(&mut self) -> u32 {
        self.message.read_u32(6)
    }

    /**
     * Get error message size
     */
    pub fn get_error_message_size(&mut self) -> u32 {
        self.message.read_u32(10)
    }

    /**
     * Get error message
     */
    pub fn get_error_message(&mut self) -> String {
        let message_size = self.message.read_u32(10);
        self.message.read_string(14, message_size as usize)
    }
}