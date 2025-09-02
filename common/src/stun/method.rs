use bincode::{Decode, Encode};
// Method is uint16 representation of 12-bit STUN method.
#[derive(Default, PartialEq, Eq, Debug, Copy, Clone, Encode, Decode)]
pub struct Method(pub u16);

// Possible methods for STUN Message.
pub const METHOD_BINDING: Method = Method(0x001);
pub const METHOD_ALLOCATE: Method = Method(0x003);
pub const METHOD_REFRESH: Method = Method(0x004);
pub const METHOD_SEND: Method = Method(0x006);
pub const METHOD_DATA: Method = Method(0x007);
pub const METHOD_CREATE_PERMISSION: Method = Method(0x008);
pub const METHOD_CHANNEL_BIND: Method = Method(0x009);

// Methods from RFC 6062.
pub const METHOD_CONNECT: Method = Method(0x000a);
pub const METHOD_CONNECTION_BIND: Method = Method(0x000b);
pub const METHOD_CONNECTION_ATTEMPT: Method = Method(0x000c);

impl Method {
    pub fn as_string(&self) -> String {
        let unknown = format!("0x{:x}", self.0);

        let s = match *self {
            METHOD_BINDING => "Binding",

            // RFC 5766
            METHOD_ALLOCATE => "Allocate(TURN)",
            METHOD_REFRESH => "Refresh(TURN)",
            METHOD_SEND => "Send(TURN)",
            METHOD_DATA => "Data(TURN)",
            METHOD_CREATE_PERMISSION => "CreatePermission(TURN)",
            METHOD_CHANNEL_BIND => "ChannelBind(TURN)",

            // RFC 6062.
            METHOD_CONNECT => "Connect(TURN)",
            METHOD_CONNECTION_BIND => "ConnectionBind(TURN)",
            METHOD_CONNECTION_ATTEMPT => "ConnectionAttempt(TURN)",
            _ => unknown.as_str(),
        };

        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_as_string() {
        assert_eq!(METHOD_BINDING.as_string(), "Binding");

        assert_eq!(METHOD_ALLOCATE.as_string(), "Allocate(TURN)");
        assert_eq!(METHOD_REFRESH.as_string(), "Refresh(TURN)");
        assert_eq!(METHOD_SEND.as_string(), "Send(TURN)");
        assert_eq!(METHOD_DATA.as_string(), "Data(TURN)");
        assert_eq!(
            METHOD_CREATE_PERMISSION.as_string(),
            "CreatePermission(TURN)"
        );

        assert_eq!(METHOD_CHANNEL_BIND.as_string(), "ChannelBind(TURN)");
        assert_eq!(METHOD_CONNECT.as_string(), "Connect(TURN)");
        assert_eq!(METHOD_CONNECTION_BIND.as_string(), "ConnectionBind(TURN)");
        assert_eq!(
            METHOD_CONNECTION_ATTEMPT.as_string(),
            "ConnectionAttempt(TURN)"
        );
        assert_eq!(Method(0x1234).as_string(), "0x1234");
    }
}
