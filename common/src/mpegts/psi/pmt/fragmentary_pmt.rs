#[cfg(test)]
mod tests;

use crate::mpegts::psi::pmt::{constants::*, PmtFields};
use crate::mpegts::psi::psi_buffer::FragmentaryPsi;
use crate::mpegts::psi::{constants::*, ProgramSpecificInformationHeader};
use crate::utils::{BitReader, DataParser, DataValidator};
use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct FragmentaryProgramMapTable {
    pub header: ProgramSpecificInformationHeader,
    pub fields: PmtFields,
    pub descriptors_payload: Vec<u8>,
    pub payload: Vec<u8>, //rest of the payload
    pub is_stuffed: bool,
}

impl PartialEq for FragmentaryProgramMapTable {
    fn eq(&self, other: &Self) -> bool {
        let header = self.header == other.header;
        let payload = self.payload == other.payload;
        let is_stuffed = self.is_stuffed == other.is_stuffed;
        let fields = self.fields == other.fields;
        let descriptors_payload = self.descriptors_payload == other.descriptors_payload;

        header && payload && is_stuffed && fields && descriptors_payload
    }
}

impl DataParser for FragmentaryProgramMapTable {
    type Output = Self;

    fn parse(data: &[u8]) -> Option<Self::Output> {
        Self::unmarshall(data, false)
    }
}

impl DataValidator for FragmentaryProgramMapTable {
    fn validate(&self) -> bool {
        self.header.section_syntax_indicator
            && self.fields.program_info_length <= MAX_PROGRAM_INFO_LENGTH
    }
}

impl FragmentaryPsi for FragmentaryProgramMapTable {
    fn unmarshall(data: &[u8], is_pointer_field: bool) -> Option<Self> {
        if data.len() < HEADER_SIZE.into() {
            return None;
        }

        let data = if is_pointer_field {
            &data[data[0] as usize + 1..]
        } else {
            data
        };

        let header = Self::unmarshall_header(data)?;
        let reader = BitReader::new(data);

        let program_number = reader.get_bits_u16(3, 0xFF, 0xFF)?;
        let pcr_pid = reader.get_pid(8, PCR_PID_UPPER_MASK)?;
        let program_info_length = reader.get_bits_u16(
            10,
            PROGRAM_INFO_LENGTH_UPPER_MASK,
            PROGRAM_INFO_LENGTH_LOWER_MASK,
        )?;

        let full_header_size: usize = (HEADER_SIZE + HEADER_AFTER_SECTION_LENGTH_SIZE).into();
        let descriptors_payload = reader.get_bytes(full_header_size, program_info_length.into())?;
        let last_byte = Self::determine_last_byte(data);
        let payload = reader.get_bytes(
            full_header_size + program_info_length as usize,
            last_byte - (full_header_size + program_info_length as usize),
        )?;

        let fields = PmtFields {
            program_number,
            pcr_pid,
            program_info_length,
        };

        Some(FragmentaryProgramMapTable {
            header,
            fields,
            descriptors_payload,
            payload,
            is_stuffed: last_byte < data.len(),
        })
    }

    fn unmarshall_header(data: &[u8]) -> Option<ProgramSpecificInformationHeader> {
        let table_id = data[0];
        let section_syntax_indicator = (data[1] & SECTION_SYNTAX_INDICATOR_MASK) != 0;
        let section_length = ((data[1] & SECTION_LENGTH_UPPER_MASK) as u16) << 8 | data[2] as u16;

        if section_length < HEADER_AFTER_SECTION_LENGTH_SIZE as u16 {
            return None;
        }

        if section_length > MAX_SECTION_LENGTH {
            return None;
        }

        let version_number = (data[5] & VERSION_NUMBER_MASK) >> 1;
        let current_next_indicator = (data[5] & CURRENT_NEXT_INDICATOR_MASK) != 0;
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
