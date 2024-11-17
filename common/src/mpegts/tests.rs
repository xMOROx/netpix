use super::*;
use crate::mpegts::header::{AdaptationFieldControl, PIDTable, TransportScramblingControl};

fn create_test_buffer(num_fragments: usize) -> Vec<u8> {
    assert!(num_fragments > 0 && num_fragments <= MAX_FRAGMENTS);
    let mut buffer = vec![0; FRAGMENT_SIZE * num_fragments];

    for i in 0..num_fragments {
        let start_index = i * FRAGMENT_SIZE;
        buffer[start_index] = SYNC_BYTE;
        buffer[start_index + 1] = 0x40; // Set PUSI
        buffer[start_index + 2] = 0x00; // PID 0x0000
        buffer[start_index + 3] = 0x10; // Payload only, CC = 0

        // Fill payload
        for j in 4..FRAGMENT_SIZE {
            buffer[start_index + j] = (j % 256) as u8;
        }
    }
    buffer
}

#[test]
fn test_unmarshall_valid_packet() {
    let buffer = create_test_buffer(7);
    let packet = MpegtsPacket::unmarshall(&buffer);
    assert!(packet.is_some(), "Failed to unmarshall packet");
    let packet = packet.unwrap();
    assert_eq!(
        packet.number_of_fragments, 7,
        "Incorrect number of fragments"
    );
    assert_eq!(
        packet.fragments.len(),
        7,
        "Incorrect number of fragments in vec"
    );

    let first_fragment = &packet.fragments[0];
    assert_eq!(first_fragment.header.pid, PIDTable::ProgramAssociation);
    assert!(first_fragment.header.payload_unit_start_indicator);
    assert!(matches!(
        first_fragment.header.adaptation_field_control,
        AdaptationFieldControl::PayloadOnly
    ));
    assert!(first_fragment.adaptation_field.is_none());
    assert!(first_fragment.payload.is_some());
    assert_eq!(
        first_fragment.payload.as_ref().unwrap().data.len(),
        FRAGMENT_SIZE - HEADER_SIZE
    );
}

#[test]
fn test_unmarshall_various_sizes() {
    for num_fragments in 1..=MAX_FRAGMENTS {
        let buffer = create_test_buffer(num_fragments);
        let packet = MpegtsPacket::unmarshall(&buffer);

        assert!(
            packet.is_some(),
            "Failed to unmarshall packet with {} fragments",
            num_fragments
        );
        let packet = packet.unwrap();
        assert_eq!(
            packet.number_of_fragments, num_fragments,
            "Incorrect number of fragments for size {}",
            num_fragments
        );
    }
}

#[test]
fn test_unmarshall_invalid_sizes() {
    // Test buffer size not multiple of FRAGMENT_SIZE
    let invalid_buffer = vec![0; FRAGMENT_SIZE + 1];
    assert!(MpegtsPacket::unmarshall(&invalid_buffer).is_none());

    // Test empty buffer
    let empty_buffer = vec![];
    assert!(MpegtsPacket::unmarshall(&empty_buffer).is_none());

    // Test too large buffer
    let large_buffer = vec![0; FRAGMENT_SIZE * (MAX_FRAGMENTS + 1)];
    assert!(MpegtsPacket::unmarshall(&large_buffer).is_none());
}

#[test]
fn test_get_header() {
    let mut buffer = vec![0; FRAGMENT_SIZE];
    buffer[0] = SYNC_BYTE;
    buffer[1] = 0b01000000; // TEI: 0, PUSI: 1, TP: 0, PID: 0x100 (upper 5 bits)
    buffer[2] = 0b01100100; // PID: 0x100 (lower 8 bits)
    buffer[3] = 0b01010000; // TSC: 01, AFC: 01, CC: 0000

    let header = MpegtsPacket::get_header(&buffer, 0).unwrap();
    assert_eq!(header.transport_error_indicator, false);
    assert_eq!(header.payload_unit_start_indicator, true);
    assert_eq!(header.transport_priority, false);
    assert_eq!(header.pid, PIDTable::PID(0x64));
    assert!(matches!(
        header.transport_scrambling_control,
        TransportScramblingControl::UserDefined(1)
    ));
    assert!(matches!(
        header.adaptation_field_control,
        AdaptationFieldControl::PayloadOnly
    ));
    assert_eq!(header.continuity_counter, 0);
}

#[test]
fn test_get_adaptation_field() {
    let mut buffer = vec![0; FRAGMENT_SIZE];
    buffer[4] = 10; // Adaptation field length

    let adaptation_field = MpegtsPacket::get_adaptation_field(&buffer, 4).unwrap();
    assert_eq!(adaptation_field.adaptation_field_length, 10);
}

#[test]
fn test_get_payload() {
    let mut buffer = vec![0; FRAGMENT_SIZE];
    for i in 4..FRAGMENT_SIZE {
        buffer[i] = i as u8;
    }

    let payload = MpegtsPacket::get_payload(&buffer, 4, 0).unwrap();
    assert_eq!(payload.data.len(), FRAGMENT_SIZE - 4);
    assert_eq!(payload.data[0], 4);
    assert_eq!(
        payload.data[payload.data.len() - 1],
        (FRAGMENT_SIZE - 1) as u8
    );
}

#[test]
fn test_get_fragment() {
    let mut buffer = create_test_buffer(1);
    buffer[1] = 0b00000000; // PID: 0x000 (upper 5 bits)
    buffer[2] = 0; // PID: 0x000 (lower 8 bits)
    buffer[3] = 0b00010000; // AFC: 01 (payload only)

    let fragment = MpegtsPacket::get_fragment(&buffer, 0, 0).unwrap();
    assert!(matches!(fragment.header.pid, PIDTable::ProgramAssociation));
    assert!(matches!(
        fragment.header.adaptation_field_control,
        AdaptationFieldControl::PayloadOnly
    ));
    assert!(fragment.adaptation_field.is_none());
    assert!(fragment.payload.is_some());
}
