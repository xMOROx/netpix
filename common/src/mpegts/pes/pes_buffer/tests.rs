use super::*;
use crate::mpegts::{Header, RawPayload};
use crate::utils::traits::BufferOperations;

#[test]
fn test_pes_packet_payload_new() {
    let payload = PesPacketPayload::new();
    assert!(payload.is_empty());
    assert!(!payload.is_completable);
    assert_eq!(payload.packet_length, 0);
}

#[test]
fn test_pes_packet_payload_set_length() {
    let mut payload = PesPacketPayload::new();
    payload.set_packet_length(100);
    assert_eq!(payload.packet_length, 100);
}

#[test]
fn test_pes_packet_payload_completable() {
    let mut payload = PesPacketPayload::new();
    payload.set_completable(true);
    assert!(payload.is_completable);
}

#[test]
fn test_pes_packet_payload_buffer_operations() {
    let mut payload = PesPacketPayload::new();
    assert!(payload.is_empty());

    payload.append(&[1, 2, 3]);
    assert_eq!(payload.get_data(), &[1, 2, 3]);

    payload.clear();
    assert!(payload.is_empty());
}

#[test]
fn test_pes_packet_payload_is_complete() {
    let mut payload = PesPacketPayload::new();

    payload.set_packet_length(4);
    payload.append(&[0, 0, 0, 0, 0, 0, 1, 2, 3, 4]);
    assert!(!payload.is_complete());

    payload.set_completable(true);
    assert!(payload.is_complete());
}

#[test]
fn test_pes_buffer_buffer_operations() {
    let mut buffer = PesBuffer::new();
    assert!(buffer.is_empty());

    buffer.append(&[1, 2, 3]);
    assert_eq!(buffer.get_data(), &[1, 2, 3]);

    buffer.clear();
    assert!(buffer.is_empty());
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
    let header = Header {
        payload_unit_start_indicator: true,
        ..Default::default()
    };

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

    let header = Header {
        payload_unit_start_indicator: true,
        ..Default::default()
    };
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
fn test_pes_buffer_complete_packet() {
    let mut buffer = PesBuffer::new();

    // Add start fragment
    let header = Header {
        payload_unit_start_indicator: true,
        ..Default::default()
    };
    let pes_data = vec![0x00, 0x00, 0x01, 0xE0, 0x00, 0x09, 0x80, 0x00, 0x00, 1, 2];

    let fragment = MpegtsFragment {
        header,
        adaptation_field: None,
        payload: Some(RawPayload { data: pes_data }),
        size: 0,
    };
    buffer.add_fragment(&fragment);

    // Add final fragment
    let header = Header::default();
    let continuation_data = vec![3, 4, 5, 6];
    let fragment = MpegtsFragment {
        header,
        adaptation_field: None,
        payload: Some(RawPayload {
            data: continuation_data,
        }),
        size: 0,
    };
    buffer.add_fragment(&fragment);

    assert!(buffer.is_complete());
    println!("{:#?}", buffer);
    let built_packet = buffer.build().unwrap();
    println!("{:#?}", built_packet);
    assert_eq!(built_packet.packet_data.unwrap(), vec![1, 2, 3, 4, 5, 6]);
}
