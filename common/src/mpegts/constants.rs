#![allow(dead_code)]
pub const FRAGMENT_SIZE: usize = 188;
pub const HEADER_SIZE: usize = 4;
pub const MAX_FRAGMENTS: usize = 7;
pub const SYNC_BYTE: u8 = 0x47;
pub const SYNC_BYTE_MASK: u8 = 0xFF;
pub const TEI_MASK: u8 = 0x80;
pub const PUSI_MASK: u8 = 0x40;
pub const TP_MASK: u8 = 0x20;
pub const PID_MASK_UPPER: u8 = 0x1F;
pub const TSC_MASK: u8 = 0xC0;
pub const AFC_MASK: u8 = 0x30;
pub const CC_MASK: u8 = 0x0F;
pub const PADDING_BYTE: u8 = 0xFF;
