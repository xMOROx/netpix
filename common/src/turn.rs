use bincode::{Decode, Encode};

#[derive(Default, Debug, Clone, Decode, Encode)]
pub struct ChannelData {
    pub data: Vec<u8>,
    pub number: u16,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<&turn::proto::chandata::ChannelData> for ChannelData {
    fn from(cd: &turn::proto::chandata::ChannelData) -> Self {
        Self {
            data: cd.data.clone(),
            number: cd.number.0,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ChannelData {
    pub fn build(packet: &super::Packet) -> Option<Self> {
        use turn::proto::chandata::ChannelData;
        let mut data = ChannelData {
            raw: packet.payload.as_deref()?.to_vec(),
            ..Default::default()
        };
        let _ = data.decode();
        Some((&data).into())
    }
}
