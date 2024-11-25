use super::*;
use crate::mpegts::{Header, RawPayload};

#[test]
fn test_pes_packet_payload_new() {
    let payload = PesPacketPayload::new();
    assert!(payload.data.is_empty());
    assert!(!payload.is_completable);
    assert_eq!(payload.packet_length, 0);
}

#[test]
fn test_pes_packet_payload_set_get_length() {
    let mut payload = PesPacketPayload::new();
    payload.set_packet_length(100);
    assert_eq!(payload.get_packet_length(), 100);
}

#[test]
fn test_pes_packet_payload_completable() {
    let mut payload = PesPacketPayload::new();
    assert!(!payload.is_completable());
    payload.set_completable(true);
    assert!(payload.is_completable());
}

#[test]
fn test_pes_packet_payload_data_operations() {
    let mut payload = PesPacketPayload::new();
    assert!(payload.is_empty());

    payload.append(&[1, 2, 3]);
    assert_eq!(payload.get_data(), &[1, 2, 3]);

    payload.get_data_mut().push(4);
    assert_eq!(payload.get_data(), &[1, 2, 3, 4]);
}

#[test]
fn test_pes_packet_payload_is_complete() {
    let mut payload = PesPacketPayload::new();
    assert!(!payload.is_complete());

    payload.set_packet_length(4);
    payload.append(&[1, 2, 3, 4]);
    assert!(!payload.is_complete());

    payload.set_completable(true);
    assert!(payload.is_complete());
}

#[test]
fn test_pes_packet_payload_clear() {
    let mut payload = PesPacketPayload::new();
    payload.append(&[1, 2, 3]);
    payload.set_completable(true);
    payload.set_packet_length(100);

    payload.clear();
    assert!(payload.is_empty());
    assert!(!payload.is_completable());
    assert_eq!(payload.get_packet_length(), 0);
}

#[test]
fn test_pes_buffer_new() {
    let buffer = PesBuffer::new();
    assert!(buffer.payload.is_empty());
}

#[test]
fn test_pes_buffer_add_fragment_without_payload() {
    let mut buffer = PesBuffer::new();
    let fragment = MpegtsFragment {
        header: Header::default(),
        adaptation_field: None,
        payload: None,
        size: 0,
    };
    buffer.add_fragment(&fragment);
    assert!(buffer.payload.is_empty());
}

#[test]
fn test_pes_buffer_add_fragment_with_start() {
    let mut buffer = PesBuffer::new();
    let mut header = Header::default();
    header.payload_unit_start_indicator = true;

    // Create minimal valid PES packet header
    let pes_data = vec![
        0x00, 0x00, 0x01, 0xE0, 0x00, 0x04, 0x80, 0x00, 0x00, 1, 2, 3, 4,
    ];

    let fragment = MpegtsFragment {
        header,
        adaptation_field: None,
        payload: Some(RawPayload { data: pes_data }),
        size: 0,
    };

    buffer.add_fragment(&fragment);
    assert!(!buffer.payload.is_empty());
    assert!(buffer.payload.is_completable());
}

#[test]
fn test_pes_buffer_add_fragment_continuation() {
    let mut buffer = PesBuffer::new();

    // First fragment with start
    let mut header = Header::default();
    header.payload_unit_start_indicator = true;
    let pes_data = vec![0x00, 0x00, 0x01, 0xE0, 0x00, 0x08, 0x80, 0x00, 0x00, 1, 2];

    let fragment = MpegtsFragment {
        header,
        adaptation_field: None,
        payload: Some(RawPayload { data: pes_data }),
        size: 0,
    };
    buffer.add_fragment(&fragment);

    // Continuation fragment
    let header = Header::default();
    let continuation_data = vec![3, 4, 5, 6, 7, 8];
    let fragment = MpegtsFragment {
        header,
        adaptation_field: None,
        payload: Some(RawPayload {
            data: continuation_data,
        }),
        size: 0,
    };
    buffer.add_fragment(&fragment);
    assert_eq!(buffer.payload.get_data().len(), 17);
}

#[test]
fn test_pes_buffer_build_incomplete() {
    let mut buffer = PesBuffer::new();
    assert!(buffer.build().is_none());
}
