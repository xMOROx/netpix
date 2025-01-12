use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq, Default)]
pub struct RawPayload {
    pub data: Vec<u8>,
    pub size: usize,
}
