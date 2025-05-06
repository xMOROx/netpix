use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct StunPacket {
    pub message_type: u16,
    pub message_length: u16,
    pub magic_cookie: u32,
    pub transaction_id: [u8; 12],
    pub attributes: Vec<StunAttribute>,
}

#[derive(Decode, Encode, Debug, Clone)]
pub struct StunAttribute {
    pub attribute_type: u16,
    pub length: u16,
    pub value: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
impl StunPacket {
    pub fn build(packet: &super::Packet) -> Option<Self> {
        let payload = packet.payload.as_ref()?;
        
        // STUN messages must be at least 20 bytes (header)
        if payload.len() < 20 {
            return None;
        }

        // Check for STUN magic cookie (0x2112A442)
        let magic_cookie = u32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);
        if magic_cookie != 0x2112A442 {
            return None;
        }

        let message_type = u16::from_be_bytes([payload[0], payload[1]]);
        let message_length = u16::from_be_bytes([payload[2], payload[3]]);
        let mut transaction_id = [0u8; 12];
        transaction_id.copy_from_slice(&payload[8..20]);

        let mut attributes = Vec::new();
        let mut offset = 20;

        while offset + 4 <= payload.len() {
            let attribute_type = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
            let length = u16::from_be_bytes([payload[offset + 2], payload[offset + 3]]);
            
            // Align to 4 bytes
            let padded_length = ((length as usize + 3) / 4) * 4;
            
            if offset + 4 + padded_length > payload.len() {
                break;
            }

            let value = payload[offset + 4..offset + 4 + length as usize].to_vec();
            
            attributes.push(StunAttribute {
                attribute_type,
                length,
                value,
            });

            offset += 4 + padded_length;
        }

        Some(Self {
            message_type,
            message_length,
            magic_cookie,
            transaction_id,
            attributes,
        })
    }
}

impl StunPacket {
    pub fn get_message_type_name(&self) -> &str {
        match self.message_type {
            0x0001 => "Binding Request",
            0x0101 => "Binding Response",
            0x0111 => "Binding Error Response",
            0x0002 => "Shared Secret Request",
            0x0102 => "Shared Secret Response",
            0x0112 => "Shared Secret Error Response",
            _ => "Unknown",
        }
    }

    pub fn get_attribute_type_name(&self, attribute_type: u16) -> &str {
        match attribute_type {
            0x0001 => "MAPPED-ADDRESS",
            0x0002 => "RESPONSE-ADDRESS",
            0x0003 => "CHANGE-REQUEST",
            0x0004 => "SOURCE-ADDRESS",
            0x0005 => "CHANGED-ADDRESS",
            0x0006 => "USERNAME",
            0x0007 => "PASSWORD",
            0x0008 => "MESSAGE-INTEGRITY",
            0x0009 => "ERROR-CODE",
            0x000A => "UNKNOWN-ATTRIBUTES",
            0x000B => "REFLECTED-FROM",
            0x0014 => "REALM",
            0x0015 => "NONCE",
            0x0020 => "XOR-MAPPED-ADDRESS",
            _ => "Unknown",
        }
    }
} 