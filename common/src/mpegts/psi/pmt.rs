pub mod constants;
pub mod fragmentary_pmt;
pub mod pmt_buffer;
pub mod stream_types;

use crate::mpegts::descriptors::Descriptors;
use crate::mpegts::psi::pmt::stream_types::StreamType;
use crate::utils::{BitReader, Crc32Reader};
use constants::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct ProgramMapTable {
    pub fields: PmtFields,
    pub descriptors: Vec<Descriptors>,
    pub elementary_streams_info: Vec<ElementaryStreamInfo>,
    pub crc_32: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct PmtFields {
    pub program_number: u16,
    pub pcr_pid: u16,
    pub program_info_length: u16,
}

impl PartialEq for ProgramMapTable {
    fn eq(&self, other: &Self) -> bool {
        let fields = self.fields == other.fields;
        let descriptors = self.descriptors == other.descriptors;
        let elementary_streams_info = self.elementary_streams_info == other.elementary_streams_info;
        let crc_32 = self.crc_32 == other.crc_32;

        fields && descriptors && elementary_streams_info && crc_32
    }
}

impl PartialEq for PmtFields {
    fn eq(&self, other: &Self) -> bool {
        let program_number = self.program_number == other.program_number;
        let pcr_pid = self.pcr_pid == other.pcr_pid;
        let program_info_length = self.program_info_length == other.program_info_length;

        program_number && pcr_pid && program_info_length
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct ElementaryStreamInfo {
    pub stream_type: StreamType, // table is defined on page 55 of H.222.0 (03/2017)
    pub elementary_pid: u16,
    pub es_info_length: u16,
    pub descriptors: Vec<Descriptors>,
}

impl PartialEq for ElementaryStreamInfo {
    fn eq(&self, other: &Self) -> bool {
        let stream_type = self.stream_type == other.stream_type;
        let elementary_pid = self.elementary_pid == other.elementary_pid;
        let es_info_length = self.es_info_length == other.es_info_length;
        let descriptors = self.descriptors == other.descriptors;

        stream_type && elementary_pid && es_info_length && descriptors
    }
}

impl ProgramMapTable {
    fn build(
        fields: PmtFields,
        descriptors_payload: &[u8],
        payload: &[u8],
    ) -> Option<ProgramMapTable> {
        let crc_reader = Crc32Reader::new(payload);

        Some(ProgramMapTable {
            fields,
            descriptors: Self::unmarshal_descriptors(descriptors_payload),
            elementary_streams_info: Self::unmarshal_elementary_streams_info(
                crc_reader.data_without_crc(),
            )?,
            crc_32: crc_reader.read_crc32()?,
        })
    }

    fn unmarshal_descriptors(data: &[u8]) -> Vec<Descriptors> {
        Descriptors::unmarshall_many(data)
    }

    fn unmarshal_elementary_streams_info(data: &[u8]) -> Option<Vec<ElementaryStreamInfo>> {
        let mut elementary_streams_info = Vec::new();
        let reader = BitReader::new(data);
        let mut offset: u16 = 0;

        while usize::from(offset + STREAM_LENGTH) <= data.len() {
            let stream_type = *reader.get_bytes(offset.into(), 1)?.first()?;
            let elementary_pid = reader.get_bits_u16(
                (offset + 1).into(),
                ELEMENTARY_PID_UPPER_MASK as u8,
                ELEMENTARY_PID_LOWER_MASK as u8,
            )?;
            let es_info_length = reader.get_bits_u16(
                (offset + 3).into(),
                ES_INFO_LENGTH_UPPER_MASK as u8,
                ES_INFO_LENGTH_LOWER_MASK as u8,
            )?;

            let descriptors_data =
                reader.get_bytes((offset + STREAM_LENGTH).into(), es_info_length as usize)?;

            elementary_streams_info.push(ElementaryStreamInfo {
                stream_type: StreamType::from(stream_type),
                elementary_pid,
                es_info_length,
                descriptors: Descriptors::unmarshall_many(&descriptors_data),
            });

            offset += STREAM_LENGTH + es_info_length;
        }

        Some(elementary_streams_info)
    }
}
