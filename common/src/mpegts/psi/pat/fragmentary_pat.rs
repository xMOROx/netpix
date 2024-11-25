#[cfg(test)]
mod tests;

use crate::mpegts::psi::pat::constants::*;
use crate::mpegts::psi::psi_buffer::FragmentaryPsi;
use crate::mpegts::psi::{
    constants::*, ProgramSpecificInformation, ProgramSpecificInformationHeader, TableId,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
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

impl FragmentaryPsi for FragmentaryProgramAssociationTable {
    fn unmarshall(data: &[u8], is_pointer_field: bool) -> Option<Self> {
        if data.len() < HEADER_SIZE {
            return None;
        }
        //TODO FIX: we are not taking into account last bytes of the previous section which resides imediately after ---> pointer_field | end of section n + 1 | new table_id | payload | ...
        // https://tsduck.io/download/docs/mpegts-introduction.pdf - more details in section `Typical section packetization`
        let data = if is_pointer_field {
            let pointer_field = data[0] as usize;
            &data[pointer_field + 1..]
        } else {
            data
        };

        let header = Self::unmarshall_header(data)?;

        let transport_stream_id = ((data[3] as u16) << 8) | data[4] as u16;

        let last_byte = Self::determine_last_byte(data);

        let payload = data[HEADER_SIZE + HEADER_AFTER_SECTION_LENGTH_SIZE..last_byte].to_vec();

        Some(FragmentaryProgramAssociationTable {
            header,
            transport_stream_id,
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
