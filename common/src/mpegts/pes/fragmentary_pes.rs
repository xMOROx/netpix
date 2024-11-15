use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct FragmentaryPes {
    pub packet_start_code_prefix: u32,
    pub stream_id: u8,
    pub pes_packet_length: u16,
}
