use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_unmarshall_with_pointer_field() {
    let data: Vec<u8> = vec![
        0x02, 0x00, 0x00, 0x00, 0xB0, 0x31, 0x00, 0x14, 0xD7, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x10,
        0x00, 0x01, 0xE0, 0x24, 0x00, 0x02, 0xE0, 0x25, 0x00, 0x03, 0xE0, 0x30, 0x00, 0x04, 0xE0,
        0x31, 0x00, 0x1A, 0xE0, 0x67, 0x00, 0x1C, 0xE0, 0x6F, 0x43, 0x9D, 0xE3, 0xF1, 0x43, 0xA3,
        0xE3, 0xF7, 0x43, 0xAC, 0xE4, 0x00, 0xC3, 0x69, 0xA6, 0xD8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ];

    let unmarshalled = FragmentaryProgramAssociationTable::unmarshall(&data, true).unwrap();
    assert_eq!(unmarshalled.header.table_id, 0);
    assert!(unmarshalled.header.section_syntax_indicator);
    assert_eq!(unmarshalled.header.section_length, 49);
    assert!(unmarshalled.header.current_next_indicator);
    assert_eq!(unmarshalled.transport_stream_id, 20);
    assert!(unmarshalled.is_stuffed);
    assert_eq!(unmarshalled.payload.len(), 44);
}

#[test]
fn test_unmarshall_without_pointer_field() {
    let data: Vec<u8> = vec![
        0x00, 0xB0, 0x31, 0x00, 0x14, 0xD7, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x10, 0x00, 0x01, 0xE0,
        0x24, 0x00, 0x02, 0xE0, 0x25, 0x00, 0x03, 0xE0, 0x30, 0x00, 0x04, 0xE0, 0x31, 0x00, 0x1A,
        0xE0, 0x67, 0x00, 0x1C, 0xE0, 0x6F, 0x43, 0x9D, 0xE3, 0xF1, 0x43, 0xA3, 0xE3, 0xF7, 0x43,
        0xAC, 0xE4, 0x00, 0xC3, 0x69, 0xA6, 0xD8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ];

    // Vector to collect the payloads from each fragment
    let unmarshalled = FragmentaryProgramAssociationTable::unmarshall(&data, false).unwrap();
    assert_eq!(unmarshalled.header.table_id, 0);
    assert!(unmarshalled.header.section_syntax_indicator);
    assert_eq!(unmarshalled.header.section_length, 49);
    assert!(unmarshalled.header.current_next_indicator);
    assert_eq!(unmarshalled.transport_stream_id, 20);
    assert!(unmarshalled.is_stuffed);
    assert_eq!(unmarshalled.payload.len(), 44);
}

#[test]
fn test_fragmentary_pat() {
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
        0xff, 0xff, 0xff,
    ];

    let unmarshalled = FragmentaryProgramAssociationTable::unmarshall(&data, true).unwrap();
    assert_eq!(unmarshalled.header.table_id, 0);
    assert!(unmarshalled.header.section_syntax_indicator);
    assert_eq!(unmarshalled.header.section_length, 13);
    assert!(unmarshalled.header.current_next_indicator);
    assert_eq!(unmarshalled.header.section_number, 0);
    assert_eq!(unmarshalled.header.version_number, 0x0f);
    assert_eq!(unmarshalled.header.last_section_number, 0x0);
    assert_eq!(unmarshalled.transport_stream_id, 3);
    assert!(unmarshalled.is_stuffed);
    assert_eq!(unmarshalled.payload.len(), 8);
}

#[test]
fn should_return_none_when_data_is_empty() {
    let data: Vec<u8> = vec![];
    assert_eq!(
        FragmentaryProgramAssociationTable::unmarshall(&data, false),
        None
    );
}

#[test]
fn should_return_none_when_data_is_too_short() {
    let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00];
    assert_eq!(
        FragmentaryProgramAssociationTable::unmarshall(&data, false),
        None
    );
}

#[test]
fn should_return_none_when_section_length_is_too_small() {
    let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00];
    assert_eq!(
        FragmentaryProgramAssociationTable::unmarshall(&data, false),
        None
    );
}

#[test]
fn should_return_none_when_section_length_is_too_large() {
    let data: Vec<u8> = vec![0x00, 0x00, 0x03, 0xFE, 0x00];
    assert_eq!(
        FragmentaryProgramAssociationTable::unmarshall(&data, false),
        None
    );
}
