use bincode::{Decode, Encode};
pub use class::MessageClass;
pub use message_type::MessageType;
pub use method::Method;
pub use stun_attribute::StunAttribute;

pub mod class;
pub mod message_type;
pub mod method;
pub mod stun_attribute;

#[derive(Decode, Encode, Debug, Clone)]
pub struct StunPacket {
    pub message_type: MessageType,
    pub message_length: u32,
    pub transaction_id: [u8; 12],
    pub attributes: Vec<StunAttribute>,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<&stun::message::Message> for StunPacket {
    fn from(msg: &stun::message::Message) -> Self {
        let message_type = MessageType::new(msg.typ.value());
        let message_length = msg.length;
        let transaction_id = msg.transaction_id.0;
        let attributes = msg.attributes.0.iter().map(StunAttribute::from).collect();

        Self {
            message_type,
            message_length,
            transaction_id,
            attributes,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl StunPacket {
    pub fn build(packet: &super::Packet) -> Option<Self> {
        use stun::message::Message;
        let buf = packet.payload.as_deref()?;
        let mut msg = Message::new();
        msg.unmarshal_binary(buf).ok()?;
        Some((&msg).into())
    }
}
