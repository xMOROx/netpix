use crate::mpegts::psi::pmt::fragmentary_pmt::FragmentaryProgramMapTable;
use crate::mpegts::psi::pmt::{PmtFields, ProgramMapTable};
use crate::mpegts::psi::psi_buffer::PsiBuffer;

pub struct PmtBuffer {
    last_section_number: u8,
    pmt_fragments: Vec<FragmentaryProgramMapTable>,
}

impl PsiBuffer<ProgramMapTable, FragmentaryProgramMapTable> for PmtBuffer {
    fn new(last_section_number: u8) -> Self {
        PmtBuffer {
            last_section_number,
            pmt_fragments: Vec::new(),
        }
    }

    fn is_complete(&self) -> bool {
        self.pmt_fragments.len() as u8 == self.last_section_number + 1
    }

    fn last_section_number(&self) -> u8 {
        self.last_section_number
    }

    fn add_fragment(&mut self, fragment: FragmentaryProgramMapTable) {
        self.pmt_fragments.push(fragment);
    }

    fn get_fragments(&self) -> &Vec<FragmentaryProgramMapTable> {
        &self.pmt_fragments
    }

    fn build(&self) -> Option<ProgramMapTable> {
        if !self.is_complete() {
            return None;
        }

        let (cumulated_payload, cumulated_descriptors_payload) = self.accumulator();
        let fields = PmtFields {
            program_number: self.pmt_fragments[0].fields.program_number,
            pcr_pid: self.pmt_fragments[0].fields.pcr_pid,
            program_info_length: self.pmt_fragments[0].fields.program_info_length,
        };

        ProgramMapTable::build(fields, &cumulated_descriptors_payload, &cumulated_payload)
    }
}

impl PmtBuffer {
    fn accumulator(&self) -> (Vec<u8>, Vec<u8>) {
        let cumulated_payload = self.pmt_fragments.iter().fold(Vec::new(), |mut acc, fragment| {
            acc.extend_from_slice(&fragment.payload);
            acc
        });


        let cumulated_descriptors_payload = self.pmt_fragments.iter().fold(Vec::new(), |mut acc, fragment| {
            acc.extend_from_slice(&fragment.descriptors_payload);
            acc
        });

        (cumulated_payload, cumulated_descriptors_payload)
    }
}


#[cfg(test)]
mod tests {
    use crate::mpegts::psi::pmt::{ElementaryStreamInfo, PmtFields};
    use crate::mpegts::psi::pmt::stream_types::StreamTypes::{AVCVideoStreamAsDefinedInItuTH264OrIsoIec1449610Video, IsoIec111723Audio, RecItuTH2220OrIsoIec138181PESPackets, RecItuTH2220OrIsoIec138181PrivateSections};
    use super::*;
    use crate::mpegts::psi::psi_buffer::FragmentaryPsi;

    #[test]
    fn test_pmt_buffer_with_one_fragment() {
        let data = vec![
            0x00, 0x02, 0xb0, 0x90,
            0x00, 0x21, 0xd5, 0x00,
            0x00, 0xe2, 0x5a, 0xf0,
            0x0b, 0x0e, 0x03, 0xc0,
            0x00, 0x00, 0x0c, 0x04,
            0x80, 0xb4, 0x81, 0x68,
            0x1b, 0xe2, 0x5a, 0xf0,
            0x16, 0x52, 0x01, 0x02,
            0x0e, 0x03, 0xc0, 0x00,
            0x00, 0x02, 0x03, 0x1a,
            0x44, 0x5f, 0x06, 0x01,
            0x02, 0x28, 0x04, 0x4d,
            0x40, 0x28, 0x3f, 0x03,
            0xe2, 0x5b, 0xf0, 0x11,
            0x52, 0x01, 0x03, 0x0e,
            0x03, 0xc0, 0x00, 0x00,
            0x0a, 0x04, 0x70, 0x6f,
            0x6c, 0x00, 0x03, 0x01,
            0x67, 0x05, 0xe2, 0x5f,
            0xf0, 0x0d, 0x52, 0x01,
            0x07, 0x0e, 0x03, 0xc0,
            0x00, 0x00, 0x6f, 0x03,
            0x00, 0x10, 0xe0, 0x06,
            0xe2, 0x5e, 0xf0, 0x12,
            0x52, 0x01, 0x06, 0x0e,
            0x03, 0xc0, 0x00, 0x00,
            0x59, 0x08, 0x70, 0x6f,
            0x6c, 0x10, 0x00, 0x02,
            0x00, 0x02, 0x06, 0xe2,
            0x60, 0xf0, 0x19, 0x52,
            0x01, 0x08, 0x0e, 0x03,
            0xc0, 0x00, 0x00, 0x0a,
            0x04, 0x61, 0x75, 0x78,
            0x03, 0x05, 0x04, 0x45,
            0x41, 0x43, 0x33, 0x7a,
            0x03, 0xc0, 0x92, 0x10,
            0x33, 0x59, 0xb6, 0x88,
            0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ];

        let mut buffer = PmtBuffer::new(0);
        let fragment = FragmentaryProgramMapTable::unmarshall(&data, true).unwrap();
        buffer.add_fragment(fragment);
        let fields = PmtFields {
            program_number: 33,
            pcr_pid: 0x025a,
            program_info_length: 11,
        };

        assert_eq!(buffer.is_complete(), true);
        assert_eq!(buffer.build(), Some(ProgramMapTable {
            fields,
            descriptors: vec![],
            elementary_streams_info: vec![
                ElementaryStreamInfo { stream_type: AVCVideoStreamAsDefinedInItuTH264OrIsoIec1449610Video, elementary_pid: 602, es_info_length: 22, descriptors: vec![] },
 ElementaryStreamInfo { stream_type: IsoIec111723Audio, elementary_pid: 603, es_info_length: 17, descriptors: vec![] },
                ElementaryStreamInfo { stream_type: RecItuTH2220OrIsoIec138181PrivateSections, elementary_pid: 607, es_info_length: 13, descriptors: vec![] },
                ElementaryStreamInfo { stream_type: RecItuTH2220OrIsoIec138181PESPackets, elementary_pid: 606, es_info_length: 18, descriptors: vec![] },
                ElementaryStreamInfo { stream_type: RecItuTH2220OrIsoIec138181PESPackets, elementary_pid: 608, es_info_length: 25, descriptors: vec![] },
            ],
            crc_32: 0x3359b688,
        }));
    }
}
