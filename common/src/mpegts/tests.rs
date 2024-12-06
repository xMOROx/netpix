use super::*;

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

    // Basic packet validation
    assert_eq!(packet.number_of_fragments, 7);
    assert_eq!(packet.fragments.len(), 7);

    // Verify first fragment structure
    let first_fragment = &packet.fragments[0];
    assert_eq!(first_fragment.header.pid, PIDTable::ProgramAssociation);
    assert!(first_fragment.header.payload_unit_start_indicator);
    assert!(matches!(
        first_fragment.header.adaptation_field_control,
        AdaptationFieldControl::PayloadOnly
    ));

    // Verify size calculations
    assert_eq!(
        first_fragment.size,
        HEADER_SIZE + first_fragment.payload.as_ref().map_or(0, |p| p.data.len())
    );
}

#[test]
fn test_process_adaptation_field() {
    let mut buffer = create_test_buffer(1);
    buffer[3] = 0b00100000; // Set AFC to adaptation field only
    buffer[4] = 10; // Adaptation field length

    let header = MpegtsPacket::get_header(&buffer, 0).unwrap();
    let result = MpegtsPacket::process_adaptation_field(&header, &buffer, 4);

    assert!(result.is_some());
    let (field, next_index) = result.unwrap();
    assert!(field.is_some());
    assert_eq!(field.unwrap().adaptation_field_length, 10);
    assert_eq!(next_index, 15); // 4 + 10 + 1
}

#[test]
fn test_process_payload() {
    let buffer = create_test_buffer(1);
    let header = MpegtsPacket::get_header(&buffer, 0).unwrap();

    let payload = MpegtsPacket::process_payload(&header, &buffer, 4, 0);
    assert!(payload.is_some());
    assert_eq!(payload.unwrap().data.len(), FRAGMENT_SIZE - 4);
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
    assert!(!header.transport_error_indicator);
    assert!(header.payload_unit_start_indicator);
    assert!(!header.transport_priority);
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
#[allow(clippy::needless_range_loop)]
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

#[test]
fn test_get_fragment_with_adaptation_field() {
    let mut buffer = create_test_buffer(1);
    buffer[3] = 0b00110000; // AFC: adaptation field + payload
    buffer[4] = 1; // Adaptation field length
    buffer[5] = 0; // Adaptation field data

    let fragment = MpegtsPacket::get_fragment(&buffer, 0, 0).unwrap();
    assert!(fragment.adaptation_field.is_some());
    assert!(fragment.payload.is_some());
    assert_eq!(
        fragment.size,
        HEADER_SIZE + 2 + fragment.payload.as_ref().unwrap().data.len()
    );
}
