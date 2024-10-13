use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct Descriptor {
    pub descriptor_tag: u8,
    pub descriptor_length: u8,
    pub data: Vec<u8>,
}

impl PartialEq for Descriptor {
    fn eq(&self, other: &Self) -> bool {
        let descriptor_tag = self.descriptor_tag == other.descriptor_tag;
        let descriptor_length = self.descriptor_length == other.descriptor_length;
        let data = self.data == other.data;

        descriptor_tag && descriptor_length && data
    }
}