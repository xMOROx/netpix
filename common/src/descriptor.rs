use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Descriptor {
    pub descriptor_tag: u8,
    pub descriptor_length: u8,
    pub data: Vec<u8>,
}