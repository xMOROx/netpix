use bincode::{Decode, Encode};
// MessageClass is 8-bit representation of 2-bit class of STUN Message Class.
#[derive(Default, PartialEq, Eq, Debug, Copy, Clone, Encode, Decode)]
pub struct MessageClass(pub u8);

// Possible values for message class in STUN Message Type.
pub const CLASS_REQUEST: MessageClass = MessageClass(0x00); // 0b00
pub const CLASS_INDICATION: MessageClass = MessageClass(0x01); // 0b01
pub const CLASS_SUCCESS_RESPONSE: MessageClass = MessageClass(0x02); // 0b10
pub const CLASS_ERROR_RESPONSE: MessageClass = MessageClass(0x03); // 0b11

impl MessageClass {
    pub fn as_string(&self) -> String {
        let s = match *self {
            CLASS_REQUEST => "Request",
            CLASS_INDICATION => "Indication",
            CLASS_SUCCESS_RESPONSE => "Success Response",
            CLASS_ERROR_RESPONSE => "Error Response",
            _ => "Unknown Message Class",
        };

        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_class_as_string() {
        assert_eq!(CLASS_REQUEST.as_string(), "Request");
        assert_eq!(CLASS_INDICATION.as_string(), "Indication");
        assert_eq!(CLASS_SUCCESS_RESPONSE.as_string(), "Success Response");
        assert_eq!(CLASS_ERROR_RESPONSE.as_string(), "Error Response");
        assert_eq!(MessageClass(0x04).as_string(), "Unknown Message Class");
    }
}
