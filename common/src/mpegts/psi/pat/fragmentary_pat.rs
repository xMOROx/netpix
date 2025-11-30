#[cfg(test)]
mod tests;

use crate::mpegts::psi::pat::constants::*;
use crate::mpegts::psi::psi_buffer::FragmentaryPsi;
use crate::mpegts::psi::{
    ProgramSpecificInformation, ProgramSpecificInformationHeader, TableId, constants::*,
};
use crate::utils::{BitReader, DataParser, DataValidator};
use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct FragmentaryProgramAssociationTable {
    pub header: ProgramSpecificInformationHeader,
    pub transport_stream_id: u16,
    pub payload: Vec<u8>,
    pub is_stuffed: bool,
}

impl ProgramSpecificInformation for FragmentaryProgramAssociationTable {
    fn get_header(&self) -> &ProgramSpecificInformationHeader {
        &self.header
    }

    fn get_table_id(&self) -> TableId {
        TableId::ProgramAssociationSection
    }
}

impl PartialEq for FragmentaryProgramAssociationTable {
    fn eq(&self, other: &Self) -> bool {
        let header = self.header == other.header;
        let transport_stream_id = self.transport_stream_id == other.transport_stream_id;
        let payload = self.payload == other.payload;
        let is_stuffed = self.is_stuffed == other.is_stuffed;

        header && transport_stream_id && payload && is_stuffed
    }
}

impl DataParser for FragmentaryProgramAssociationTable {
    type Output = Self;

    fn parse(data: &[u8]) -> Option<Self::Output> {
        Self::unmarshall(data, false)
    }
}

impl DataValidator for FragmentaryProgramAssociationTable {
    fn validate(&self) -> bool {
        self.header.table_id == TableId::ProgramAssociationSection as u8
            && self.header.section_syntax_indicator
    }
}

impl FragmentaryPsi for FragmentaryProgramAssociationTable {
    fn unmarshall(data: &[u8], is_pointer_field: bool) -> Option<Self> {
        if data.len() < HEADER_SIZE {
            return None;
        }
        //TODO FIX: we are not taking into account last bytes of the previous section which resides immediately after ---> pointer_field | end of section n + 1 | new table_id | payload | ...
        // https://tsduck.io/download/docs/mpegts-introduction.pdf - more details in section `Typical section packetization`
        let data = if is_pointer_field {
            &data[data[0] as usize + 1..]
        } else {
            data
        };

        let header = Self::unmarshall_header(data)?;
        let reader = BitReader::new(data);

        let transport_stream_id = reader.get_bits_u16(3, 0xFF, 0xFF)?;
        let last_byte = Self::determine_last_byte(data);
        let payload = reader.get_bytes(
            HEADER_SIZE + HEADER_AFTER_SECTION_LENGTH_SIZE,
            last_byte - (HEADER_SIZE + HEADER_AFTER_SECTION_LENGTH_SIZE),
        )?;

        Some(FragmentaryProgramAssociationTable {
            header,
            transport_stream_id,
            payload,
            is_stuffed: last_byte < data.len(),
        })
    }

    fn unmarshall_header(data: &[u8]) -> Option<ProgramSpecificInformationHeader> {
        let reader = BitReader::new(data);

        let table_id = data[0];
        let section_syntax_indicator = reader.get_bit(1, 7)?;
        let section_length = reader.get_bits_u16(1, SECTION_LENGTH_UPPER_MASK, 0xFF)?;

        if !Self::validate_section_length(section_length) {
            return None;
        }

        let version_number = reader.get_bits(5, VERSION_NUMBER_MASK, 1)?;
        let current_next_indicator = reader.get_bit(5, 0)?;
        let section_number = data[6];
        let last_section_number = data[7];

        Some(ProgramSpecificInformationHeader {
            table_id,
            section_syntax_indicator,
            section_length,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
        })
    }
}

impl FragmentaryProgramAssociationTable {
    fn validate_section_length(section_length: u16) -> bool {
        section_length >= HEADER_AFTER_SECTION_LENGTH_SIZE as u16
            && section_length <= MAX_SECTION_LENGTH
    }
}
