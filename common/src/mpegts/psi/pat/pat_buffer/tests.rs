use super::*;
use crate::mpegts::psi::pat::ProgramAssociationItem;
use crate::mpegts::psi::psi_buffer::FragmentaryPsi;
use pretty_assertions::assert_eq;

#[test]
fn test_pat_buffer_with_one_fragment() {
    let data: Vec<u8> = vec![
        0x00, 0x00, 0xb0, 0x0d, 0x00, 0x03, 0xdf, 0x00, 0x00, 0x00, 0x23, 0xed, 0xad, 0x5a, 0xe9,
        0x7d, 0xda, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];

    let fragment = FragmentaryProgramAssociationTable::unmarshall(&data, true).unwrap();
    let mut buffer = PatBuffer::new(fragment.header.last_section_number);

    buffer.add_fragment(fragment);

    assert!(buffer.is_complete());
    assert_eq!(
        buffer.build(),
        Some(ProgramAssociationTable {
            transport_stream_id: 3,
            programs: vec![ProgramAssociationItem {
                program_number: 0x0023,
                network_pid: None,
                program_map_pid: Some(0x0dad),
            },],
            crc_32: 0x5ae97dda,
            fragment_count: 1,
        })
    );
}

#[test]
fn test_pat_buffer_with_two_fragments() {
    let data1: Vec<u8> = vec![
        // pointer_field, table_id, section_syntax_indicator, section_length
        0x00, /* header */ 0x00, 0xb0, 0xc4, /* header */
        /*section*/ 0x12, 0x95, 0xc7, 0x00, 0x01, /*section*/
        0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed,
        0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23,
        0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00,
        0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad,
        0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed,
        0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23,
        0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00,
        0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad,
        0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed,
        0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23,
        0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00,
        0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed,
    ];

    let data2: Vec<u8> = vec![
        /* header */ 0x00, 0xb0, 0xc4, /* header */
        /*section*/ 0x12, 0x95, 0xc7, 0x01, 0x01, /*section*/
        0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x00, 0x23, 0xed, 0xad, 0x5a, 0xe9,
        0x7d, 0xda, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ];

    let fragment1 = FragmentaryProgramAssociationTable::unmarshall(&data1, true).unwrap();
    let fragment2 = FragmentaryProgramAssociationTable::unmarshall(&data2, false).unwrap();
    let mut buffer = PatBuffer::new(fragment1.header.last_section_number);

    let frag1 = fragment1.clone();
    let frag2 = fragment2.clone();

    buffer.add_fragment(fragment1);
    buffer.add_fragment(fragment2);

    let pat = buffer.build().unwrap();

    let mut programs = Vec::new();

    for _ in 0..48 {
        programs.push(ProgramAssociationItem {
            program_number: 35,
            network_pid: None,
            program_map_pid: Some(3501),
        });
    }

    assert!(buffer.is_complete());
    assert_eq!(buffer.last_section_number, 1);
    assert_eq!(pat.transport_stream_id, 4757);
    assert_eq!(frag1.header.section_length, 196);
    assert_eq!(frag2.header.section_length, 196);
    assert_eq!(frag1.header.section_number, 0);
    assert_eq!(frag1.header.last_section_number, 1);
    assert_eq!(frag2.header.section_number, 1);
    assert_eq!(frag2.header.last_section_number, 1);
    assert_eq!(pat.crc_32, 0x5ae97dda);
    assert_eq!(pat.programs.len(), programs.len());
    assert_eq!(pat.programs, programs);
}
