pub struct PrivateSection {
    pub table_id: u8,
    pub section_syntax_indicator: bool,
    pub private_indicator: bool,
    pub private_section_length: u16,
    pub data: Vec<u8>,
    pub table_id_extension: Option<u16>,
    pub version_number: Option<u8>,
    pub current_next_indicator: Option<bool>,
    pub section_number: Option<u8>,
    pub last_section_number: Option<u8>,
    pub crc_32: u32,
}