pub(super) const PACKET_START_CODE_PREFIX: u32 = 0x000001;
pub(super) const REQUIRED_FIELDS_SIZE: usize = 6; // in bytes
pub(super) const HEADER_REQUIRED_FIELDS_SIZE: usize = 3; // in bytes
pub(super) const HEADER_MANDATORY_BITS_MASK: u8 = 0xc0;
pub(super) const HEADER_MANDATORY_BITS_VALUE: u8 = 0b10;

pub(super) const SCRAMBLING_CONTROL_MASK: u8 = 0x30;
pub(super) const PRIORITY_MASK: u8 = 0x08;
pub(super) const DATA_ALIGNMENT_MASK: u8 = 0x04;
pub(super) const COPYRIGHT_MASK: u8 = 0x02;
pub(super) const ORIGINAL_MASK: u8 = 0x01;
pub(super) const PTS_DTS_FLAGS_MASK: u8 = 0xC0;
pub(super) const ESCR_FLAG_MASK: u8 = 0x20;
pub(super) const ES_RATE_FLAG_MASK: u8 = 0x10;
pub(super) const DSM_TRICK_MODE_FLAG_MASK: u8 = 0x08;
pub(super) const ADDITIONAL_COPY_INFO_FLAG_MASK: u8 = 0x04;
pub(super) const PES_CRC_FLAG_MASK: u8 = 0x02;
pub(super) const PES_EXTENSION_FLAG_MASK: u8 = 0x01;

pub(super) const PTS_DTS_REQUIRED_BITS_MASK: u8 = 0xF0;
pub(super) const ONLY_PTS_REQUIRED_BITS_VALUE: u8 = 0b0010;
pub(super) const PTS_AND_DTS_REQUIRED_BITS_FIRST_VALUE: u8 = 0b0011;
pub(super) const PTS_AND_DTS_REQUIRED_BITS_SECOND_VALUE: u8 = 0b0001;
pub(super) const MARKER_BIT: u8 = 0x01;
