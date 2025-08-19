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
            METHOD_ALLOCATE => "Allocate",
            METHOD_REFRESH => "Refresh",
            METHOD_SEND => "Send",
            METHOD_DATA => "Data",
            METHOD_CREATE_PERMISSION => "CreatePermission",
            METHOD_CHANNEL_BIND => "ChannelBind",

            // RFC 6062.
            METHOD_CONNECT => "Connect",
            METHOD_CONNECTION_BIND => "ConnectionBind",
            METHOD_CONNECTION_ATTEMPT => "ConnectionAttempt",
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
        assert_eq!(METHOD_ALLOCATE.as_string(), "Allocate");
        assert_eq!(METHOD_REFRESH.as_string(), "Refresh");
        assert_eq!(METHOD_SEND.as_string(), "Send");
        assert_eq!(METHOD_DATA.as_string(), "Data");
        assert_eq!(METHOD_CREATE_PERMISSION.as_string(), "CreatePermission");
        assert_eq!(METHOD_CHANNEL_BIND.as_string(), "ChannelBind");
        assert_eq!(METHOD_CONNECT.as_string(), "Connect");
        assert_eq!(METHOD_CONNECTION_BIND.as_string(), "ConnectionBind");
        assert_eq!(METHOD_CONNECTION_ATTEMPT.as_string(), "ConnectionAttempt");
        assert_eq!(Method(0x1234).as_string(), "0x1234");
    }
}
