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