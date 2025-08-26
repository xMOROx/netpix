use bincode::{Decode, Encode};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Decode, Encode, Debug, Clone)]
pub struct StunAttribute {
    pub attribute_type: u16,
    pub length: u16,
    pub value: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<&stun::attributes::RawAttribute> for StunAttribute {
    fn from(attr: &stun::attributes::RawAttribute) -> Self {
        Self {
            attribute_type: attr.typ.0,
            length: attr.length,
            value: attr.value.clone(),
        }
    }
}

impl StunAttribute {
    pub fn as_string(&self) -> String {
        let value = self.get_value();
        if value.is_empty() {
            format!("{}", self.get_type_name())
        } else {
            format!("{}: {}", self.get_type_name(), value)
        }
    }

    pub fn get_type_name(&self) -> String {
        let other = format!("0x{:04x}", self.attribute_type);

        let s = match self.attribute_type {
            ATTR_MAPPED_ADDRESS => "MAPPED-ADDRESS",
            ATTR_USERNAME => "USERNAME",
            ATTR_ERROR_CODE => "ERROR-CODE",
            ATTR_MESSAGE_INTEGRITY => "MESSAGE-INTEGRITY",
            ATTR_UNKNOWN_ATTRIBUTES => "UNKNOWN-ATTRIBUTES",
            ATTR_REALM => "REALM",
            ATTR_NONCE => "NONCE",
            ATTR_XORMAPPED_ADDRESS => "XOR-MAPPED-ADDRESS",
            ATTR_SOFTWARE => "SOFTWARE",
            ATTR_ALTERNATE_SERVER => "ALTERNATE-SERVER",
            ATTR_FINGERPRINT => "FINGERPRINT",
            ATTR_PRIORITY => "PRIORITY",
            ATTR_USE_CANDIDATE => "USE-CANDIDATE",
            ATTR_ICE_CONTROLLED => "ICE-CONTROLLED",
            ATTR_ICE_CONTROLLING => "ICE-CONTROLLING",
            ATTR_CHANNEL_NUMBER => "CHANNEL-NUMBER",
            ATTR_LIFETIME => "LIFETIME",
            ATTR_XOR_PEER_ADDRESS => "XOR-PEER-ADDRESS",
            ATTR_DATA => "DATA",
            ATTR_XOR_RELAYED_ADDRESS => "XOR-RELAYED-ADDRESS",
            ATTR_EVEN_PORT => "EVEN-PORT",
            ATTR_REQUESTED_TRANSPORT => "REQUESTED-TRANSPORT",
            ATTR_DONT_FRAGMENT => "DONT-FRAGMENT",
            ATTR_RESERVATION_TOKEN => "RESERVATION-TOKEN",
            ATTR_CONNECTION_ID => "CONNECTION-ID",
            ATTR_REQUESTED_ADDRESS_FAMILY => "REQUESTED-ADDRESS-FAMILY",
            ATTR_MESSAGE_INTEGRITY_SHA256 => "MESSAGE-INTEGRITY-SHA256",
            ATTR_PASSWORD_ALGORITHM => "PASSWORD-ALGORITHM",
            ATTR_USER_HASH => "USERHASH",
            ATTR_PASSWORD_ALGORITHMS => "PASSWORD-ALGORITHMS",
            ATTR_ALTERNATE_DOMAIN => "ALTERNATE-DOMAIN",
            _ => other.as_str(),
        };

        s.to_string()
    }
    pub fn as_string_with_txid(&self, txid: &[u8; 12]) -> String {
        let value = self.get_value_with_txid(txid);
        if value.is_empty() {
            format!("{}", self.get_type_name())
        } else {
            format!("{}: {}", self.get_type_name(), value)
        }
    }

    pub fn get_value_with_txid(&self, txid: &[u8; 12]) -> String {
        match self.attribute_type {
            // Delegate to the existing branches but use helpers and txid where needed.
            ATTR_USERNAME | ATTR_REALM | ATTR_NONCE | ATTR_SOFTWARE => {
                if let Ok(str_value) = std::str::from_utf8(&self.value) {
                    str_value.to_string()
                } else {
                    "<invalid utf-8>".to_string()
                }
            }
            ATTR_ERROR_CODE => {
                if self.value.len() >= 4 {
                    let code = ((self.value[2] as u16) * 100) + (self.value[3] as u16);
                    format!("{}", code)
                } else {
                    "<invalid length>".to_string()
                }
            }
            ATTR_MAPPED_ADDRESS => match self.value.get(1).copied() {
                Some(0x01) => parse_address_v4(&self.value)
                    .map(|(ip, port)| format!("{ip}:{port}"))
                    .unwrap_or_else(|| "<invalid length>".to_string()),
                Some(0x02) => parse_address_v6(&self.value)
                    .map(|(ip, port)| format!("[{ip}]:{port}"))
                    .unwrap_or_else(|| "<invalid length>".to_string()),
                _ => "<unknown family>".to_string(),
            },
            ATTR_XORMAPPED_ADDRESS | ATTR_XOR_PEER_ADDRESS | ATTR_XOR_RELAYED_ADDRESS => {
                match self.value.get(1).copied() {
                    Some(0x01) => parse_xor_address_v4(&self.value)
                        .map(|(ip, port)| format!("{ip}:{port}"))
                        .unwrap_or_else(|| "<invalid length>".to_string()),
                    Some(0x02) => parse_xor_address_v6(&self.value, txid)
                        .map(|(ip, port)| format!("[{ip}]:{port}"))
                        .unwrap_or_else(|| "<invalid length>".to_string()),
                    _ => "<unknown family>".to_string(),
                }
            }
            _ => self
                .value
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>(),
        }
    }

    // Keep old get_value() for compatibility, but prefer get_value_with_txid in call-sites.
    pub fn get_value(&self) -> String {
        // Best effort without transaction ID; IPv6 XOR decoding may be incorrect.
        self.get_value_with_txid(&[0u8; 12])
    }
}

// Small helpers to reduce duplication and ensure proper bounds checks.
fn parse_address_v4(v: &[u8]) -> Option<(Ipv4Addr, u16)> {
    if v.len() < 8 {
        return None;
    }
    let port = ((v[2] as u16) << 8) | (v[3] as u16);
    let ip = Ipv4Addr::new(v[4], v[5], v[6], v[7]);
    Some((ip, port))
}

fn parse_address_v6(v: &[u8]) -> Option<(Ipv6Addr, u16)> {
    if v.len() < 20 {
        return None;
    }
    let port = ((v[2] as u16) << 8) | (v[3] as u16);
    let mut seg = [0u16; 8];
    for i in 0..8 {
        let hi = v[4 + i * 2] as u16;
        let lo = v[5 + i * 2] as u16;
        seg[i] = (hi << 8) | lo;
    }
    let ip = Ipv6Addr::new(
        seg[0], seg[1], seg[2], seg[3], seg[4], seg[5], seg[6], seg[7],
    );
    Some((ip, port))
}

fn parse_xor_address_v4(v: &[u8]) -> Option<(Ipv4Addr, u16)> {
    if v.len() < 8 {
        return None;
    }
    let x_port = ((v[2] as u16) << 8) | (v[3] as u16);
    let port = x_port ^ ((MAGIC_COOKIE >> 16) as u16);
    let x_addr = u32::from_be_bytes([v[4], v[5], v[6], v[7]]);
    let ip = Ipv4Addr::from(x_addr ^ MAGIC_COOKIE);
    Some((ip, port))
}

fn parse_xor_address_v6(v: &[u8], txid: &[u8; 12]) -> Option<(Ipv6Addr, u16)> {
    if v.len() < 20 {
        return None;
    }
    // XOR port with first 16 bits of magic cookie (RFC 5389 §15.2 / §15.1)
    let x_port = ((v[2] as u16) << 8) | (v[3] as u16);
    let port = x_port ^ ((MAGIC_COOKIE >> 16) as u16);

    // Build 128-bit mask: magic cookie (32-bit) || transaction ID (96-bit)
    let mut mask = [0u8; 16];
    mask[..4].copy_from_slice(&MAGIC_COOKIE.to_be_bytes());
    mask[4..].copy_from_slice(txid);

    // XOR the IPv6 address bytes (bytes 4..20) with the mask
    let mut ip_bytes = [0u8; 16];
    ip_bytes.copy_from_slice(&v[4..20]);
    for i in 0..16 {
        ip_bytes[i] ^= mask[i];
    }
    let ip = Ipv6Addr::from(ip_bytes);
    Some((ip, port))
}

/// Attributes from comprehension-required range (0x0000-0x7FFF.
pub const ATTR_MAPPED_ADDRESS: u16 = 0x0001; // MAPPED-ADDRESS
pub const ATTR_USERNAME: u16 = 0x0006; // USERNAME
pub const ATTR_MESSAGE_INTEGRITY: u16 = 0x0008; // MESSAGE-INTEGRITY
pub const ATTR_ERROR_CODE: u16 = 0x0009; // ERROR-CODE
pub const ATTR_UNKNOWN_ATTRIBUTES: u16 = 0x000A; // UNKNOWN-ATTRIBUTES
pub const ATTR_REALM: u16 = 0x0014; // REALM
pub const ATTR_NONCE: u16 = 0x0015; // NONCE
pub const ATTR_XORMAPPED_ADDRESS: u16 = 0x0020; // XOR-MAPPED-ADDRESS

/// Attributes from comprehension-optional range (0x8000-0xFFFF.
pub const ATTR_SOFTWARE: u16 = 0x8022; // SOFTWARE
pub const ATTR_ALTERNATE_SERVER: u16 = 0x8023; // ALTERNATE-SERVER
pub const ATTR_FINGERPRINT: u16 = 0x8028; // FINGERPRINT

/// Attributes from RFC 5245 ICE.
pub const ATTR_PRIORITY: u16 = 0x0024; // PRIORITY
pub const ATTR_USE_CANDIDATE: u16 = 0x0025; // USE-CANDIDATE
pub const ATTR_ICE_CONTROLLED: u16 = 0x8029; // ICE-CONTROLLED
pub const ATTR_ICE_CONTROLLING: u16 = 0x802A; // ICE-CONTROLLING

/// Attributes from RFC 5766 TURN.
pub const ATTR_CHANNEL_NUMBER: u16 = 0x000C; // CHANNEL-NUMBER
pub const ATTR_LIFETIME: u16 = 0x000D; // LIFETIME
pub const ATTR_XOR_PEER_ADDRESS: u16 = 0x0012; // XOR-PEER-ADDRESS
pub const ATTR_DATA: u16 = 0x0013; // DATA
pub const ATTR_XOR_RELAYED_ADDRESS: u16 = 0x0016; // XOR-RELAYED-ADDRESS
pub const ATTR_EVEN_PORT: u16 = 0x0018; // EVEN-PORT
pub const ATTR_REQUESTED_TRANSPORT: u16 = 0x0019; // REQUESTED-TRANSPORT
pub const ATTR_DONT_FRAGMENT: u16 = 0x001A; // DONT-FRAGMENT
pub const ATTR_RESERVATION_TOKEN: u16 = 0x0022; // RESERVATION-TOKEN

/// Attributes from RFC 5780 NAT Behavior Discovery
pub const ATTR_CHANGE_REQUEST: u16 = 0x0003; // CHANGE-REQUEST
pub const ATTR_PADDING: u16 = 0x0026; // PADDING
pub const ATTR_RESPONSE_PORT: u16 = 0x0027; // RESPONSE-PORT
pub const ATTR_CACHE_TIMEOUT: u16 = 0x8027; // CACHE-TIMEOUT
pub const ATTR_RESPONSE_ORIGIN: u16 = 0x802b; // RESPONSE-ORIGIN
pub const ATTR_OTHER_ADDRESS: u16 = 0x802C; // OTHER-ADDRESS

/// Attributes from RFC 3489, removed by RFC 5389,
///  but still used by RFC5389-implementing software like Vovida.org, reTURNServer, etc.
pub const ATTR_SOURCE_ADDRESS: u16 = 0x0004; // SOURCE-ADDRESS
pub const ATTR_CHANGED_ADDRESS: u16 = 0x0005; // CHANGED-ADDRESS

/// Attributes from RFC 6062 TURN Extensions for TCP Allocations.
pub const ATTR_CONNECTION_ID: u16 = 0x002a; // CONNECTION-ID

/// Attributes from RFC 6156 TURN IPv6.
pub const ATTR_REQUESTED_ADDRESS_FAMILY: u16 = 0x0017; // REQUESTED-ADDRESS-FAMILY

/// Attributes from An Origin Attribute for the STUN Protocol.
pub const ATTR_ORIGIN: u16 = 0x802F;

/// Attributes from RFC 8489 STUN.
pub const ATTR_MESSAGE_INTEGRITY_SHA256: u16 = 0x001C; // MESSAGE-INTEGRITY-SHA256
pub const ATTR_PASSWORD_ALGORITHM: u16 = 0x001D; // PASSWORD-ALGORITHM
pub const ATTR_USER_HASH: u16 = 0x001E; // USER-HASH
pub const ATTR_PASSWORD_ALGORITHMS: u16 = 0x8002; // PASSWORD-ALGORITHMS
pub const ATTR_ALTERNATE_DOMAIN: u16 = 0x8003; // ALTERNATE-DOMAIN

pub const MAGIC_COOKIE: u32 = 0x2112A442;
