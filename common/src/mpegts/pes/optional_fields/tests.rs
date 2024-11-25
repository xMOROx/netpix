use super::*;
use crate::utils::{BitReader, TimestampReader};

// Test timestamp reading using TimestampReader
#[test]
fn test_optional_fields_timestamp() {
    let data = [0b00001011, 0b10110011, 0b11101001, 0b10110011, 0b10000011];
    let reader = TimestampReader::new(&data);
    let result = reader.read_timestamp();
    assert_eq!(result, Ok(6_123_313_601));
}

#[test]
fn test_optional_fields_unmarshall_pts_dts_invalid() {
    let data = [0b00001011, 0b10110011, 0b11101001, 0b10110011];
    let result = OptionalFields::unmarshall_pts_dts(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_optional_fields_unmarshall_pts_dts_invalid_first_marker_bit() {
    let data = [0b00001010, 0b10110011, 0b11101001, 0b10110011, 0b10000011];
    let result = OptionalFields::unmarshall_pts_dts(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_optional_fields_unmarshall_pts_dts_invalid_second_marker_bit() {
    let data = [0b00001011, 0b10110011, 0b11101000, 0b10110011, 0b10000011];
    let result = OptionalFields::unmarshall_pts_dts(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_optional_fields_unmarshall_pts_dts_invalid_third_marker_bit() {
    let data = [0b00001011, 0b10110011, 0b11101001, 0b10110011, 0b10000010];
    let result = OptionalFields::unmarshall_pts_dts(&data);
    assert_eq!(result, Err(()));
}

// Test ESCR reading using TimestampReader
#[test]
fn test_optional_fields_escr() {
    let data = [
        0b11111111, // [reserved:2][base 32-30:3][marker:1][base 29-27:2]
        0b11111111, // [base 26-19:8]
        0b11111111, // [marker:1][base 18-12:7][base 11:1]
        0b11111111, // [base 10-3:8]
        0b11111111, // [marker:1][base 2-0:3][marker:1][ext 8-6:3]
        0b11111111, // [ext 5-0:6][marker:1]
    ];
    let reader = TimestampReader::new(&data);
    let result = reader.read_escr();
    assert_eq!(result, Ok((8_589_934_591, 511)));
}

#[test]
fn test_optional_fields_unmarshall_escr_invalid() {
    let data = [0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
    let result = OptionalFields::unmarshall_escr(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_optional_fields_unmarshall_escr_invalid_first_marker_bit() {
    let data = [
        0b00111011, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
    ];
    let result = OptionalFields::unmarshall_escr(&data);
    assert_eq!(result, Err(()));
}
#[test]
fn test_optional_fields_unmarshall_escr_invalid_second_marker_bit() {
    let data = [
        0b00111111, 0b11111111, 0b11111011, 0b11111111, 0b11111111, 0b11111111,
    ];
    let result = OptionalFields::unmarshall_escr(&data);
    assert_eq!(result, Err(()));
}
#[test]
fn test_optional_fields_unmarshall_escr_invalid_third_marker_bit() {
    let data = [
        0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111011, 0b11111111,
    ];
    let result = OptionalFields::unmarshall_escr(&data);
    assert_eq!(result, Err(()));
}
#[test]
fn test_optional_fields_unmarshall_escr_invalid_fourth_marker_bit() {
    let data = [
        0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111110,
    ];
    let result = OptionalFields::unmarshall_escr(&data);
    assert_eq!(result, Err(()));
}

// Test ES rate reading using BitReader
#[test]
fn test_optional_fields_es_rate() {
    let data = [0b11111111, 0b11111111, 0b11111111];
    let reader = BitReader::new(&data);
    let result = reader.get_bits(0, 0x7F, 0).map(|upper| {
        ((upper as u32) << 15) | ((data[1] as u32) << 7) | ((data[2] as u32 & 0xFE) >> 1)
    });
    assert_eq!(result, Some(4_194_303));
}

#[test]
fn test_optional_fields_unmarshall_es_rate_invalid() {
    let data = [0b11111111, 0b11111111];
    let result = OptionalFields::unmarshall_es_rate(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_optional_fields_unmarshall_es_rate_invalid_first_marker_bit() {
    let data = [0b01111111, 0b11111111, 0b11111111];
    let result = OptionalFields::unmarshall_es_rate(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_optional_fields_unmarshall_es_rate_invalid_second_marker_bit() {
    let data = [0b11111111, 0b11111111, 0b11111110];
    let result = OptionalFields::unmarshall_es_rate(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_unmarshall_tref() {
    let data = [0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
    let reader = TimestampReader::new(&data);
    let result = reader.read_tref();
    assert_eq!(result, Ok(8_589_934_591));
}

#[test]
fn test_unmarshall_tref_invalid() {
    let data = [0b11111111, 0b11111111, 0b11111111, 0b11111111];
    let reader = TimestampReader::new(&data);
    let result = reader.read_tref();
    assert_eq!(result, Err(()));
}

#[test]
fn test_unmarshall_tref_invalid_first_marker_bit() {
    let data = [0b11111110, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
    let reader = TimestampReader::new(&data);
    let result = reader.read_tref();
    assert_eq!(result, Err(()));
}

#[test]
fn test_unmarshall_tref_invalid_second_marker_bit() {
    let data = [0b11111111, 0b11111111, 0b11111110, 0b11111111, 0b11111111];
    let reader = TimestampReader::new(&data);
    let result = reader.read_tref();
    assert_eq!(result, Err(()));
}

#[test]
fn test_unmarshall_tref_invalid_third_marker_bit() {
    let data = [0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111110];
    let reader = TimestampReader::new(&data);
    let result = reader.read_tref();
    assert_eq!(result, Err(()));
}

#[test]
fn test_context_flags_parsing() {
    let data = [0b11010101];
    let flags = ContextFlags::parse(&data).unwrap();
    assert_eq!(flags.pts_dts_flags, 0b11);
    assert!(!flags.escr_flag);
    assert!(flags.es_rate_flag);
    assert!(!flags.dsm_trick_mode_flag);
    assert!(flags.additional_copy_info_flag);
    assert!(!flags.pes_crc_flag);
    assert!(flags.pes_extension_flag);
}

#[test]
fn test_optional_fields_pts_only() {
    // PTS only (0x21 = '00100001')
    let data = [
        0x21, 0x00, 0x01, 0x00, 0x03, // PTS
    ];
    let context = ContextFlagsBuilder::new().with_pts_dts_flags(0b10).build();

    let (fields, consumed) = OptionalFields::unmarshall(&data, context).unwrap();
    assert_eq!(consumed, 5);
    assert_eq!(fields.pts, Some(1));
    assert_eq!(fields.dts, None);
}

#[test]
fn test_optional_fields_pts_dts() {
    // PTS and DTS (0x31 = '00110001')
    let data = [
        0x31, 0x00, 0x01, 0x00, 0x03, // PTS
        0x11, 0x00, 0x01, 0x00, 0x03, // DTS
    ];
    let context = ContextFlagsBuilder::new().with_pts_dts_flags(0b11).build();

    let (fields, consumed) = OptionalFields::unmarshall(&data, context).unwrap();
    assert_eq!(consumed, 10);
    assert_eq!(fields.pts, Some(1));
    assert_eq!(fields.dts, Some(1));
}

// #[test]
// fn test_optional_fields_escr() {
//     let data = [
//         0b00000100, // [reserved:2][base 32-30:3][marker:1][base 29-27:2]
//         0b00000000, // [base 26-19:8]
//         0b00000100, // [marker:1][base 18-12:7][base 11:1]
//         0b00000000, // [base 10-3:8]
//         0b00000100, // [marker:1][base 2-0:3][marker:1][ext 8-6:3]
//         0b00000001, // [ext 5-0:6][marker:1]
//     ];
//     let context = ContextFlagsBuilder::new().with_escr_flag(true).build();

//     let (fields, consumed) = OptionalFields::unmarshall(&data, context).unwrap();
//     assert_eq!(consumed, 6);
//     assert!(fields.escr_base.is_some());
//     assert!(fields.escr_extension.is_some());
// }

// #[test]
// fn test_optional_fields_es_rate() {
//     let data = [
//         0x80, 0x00, 0x01, // ES rate
//     ];
//     let context = ContextFlagsBuilder::new().with_es_rate_flag(true).build();

//     let (fields, consumed) = OptionalFields::unmarshall(&data, context).unwrap();
//     assert_eq!(consumed, 3);
//     assert!(fields.es_rate.is_some());
// }

#[test]
fn test_optional_fields_trick_mode() {
    let data = [0x01]; // Fast forward with field_id 01
    let context = ContextFlagsBuilder::new()
        .with_dsm_trick_mode_flag(true)
        .build();

    let (fields, consumed) = OptionalFields::unmarshall(&data, context).unwrap();
    assert_eq!(consumed, 1);
    assert!(fields.trick_mode_control.is_some());
}

#[test]
fn test_optional_fields_additional_copy_info() {
    let data = [0x81]; // Copy info with marker bit
    let context = ContextFlagsBuilder::new()
        .with_additional_copy_info_flag(true)
        .build();

    let (fields, consumed) = OptionalFields::unmarshall(&data, context).unwrap();
    assert_eq!(consumed, 1);
    assert!(fields.additional_copy_info.is_some());
}

#[test]
fn test_optional_fields_crc() {
    let data = [0x12, 0x34]; // CRC value
    let context = ContextFlagsBuilder::new().with_pes_crc_flag(true).build();

    let (fields, consumed) = OptionalFields::unmarshall(&data, context).unwrap();
    assert_eq!(consumed, 2);
    assert_eq!(fields.previous_pes_packet_crc, Some(0x1234));
}

#[test]
fn test_optional_fields_validation() {
    let invalid_fields = OptionalFields {
        size: 0,
        pts: None,
        dts: Some(1), // Invalid: DTS without PTS
        escr_base: None,
        escr_extension: None,
        es_rate: None,
        trick_mode_control: None,
        additional_copy_info: None,
        previous_pes_packet_crc: None,
        pes_extension_data: None,
    };
    assert!(!invalid_fields.validate());

    let invalid_fields2 = OptionalFields {
        size: 0,
        pts: Some(1),
        dts: Some(2), // Invalid: DTS > PTS
        escr_base: None,
        escr_extension: None,
        es_rate: None,
        trick_mode_control: None,
        additional_copy_info: None,
        previous_pes_packet_crc: None,
        pes_extension_data: None,
    };
    assert!(!invalid_fields2.validate());

    let valid_fields = OptionalFields {
        size: 0,
        pts: Some(2),
        dts: Some(1), // Valid: PTS > DTS
        escr_base: None,
        escr_extension: None,
        es_rate: None,
        trick_mode_control: None,
        additional_copy_info: None,
        previous_pes_packet_crc: None,
        pes_extension_data: None,
    };
    assert!(valid_fields.validate());
}

#[test]
fn test_invalid_marker_bits() {
    // Test PTS/DTS with invalid marker bits
    let data = [
        0x21, 0x00, 0x00, 0x00, 0x00, // All marker bits are 0 (invalid)
    ];
    let context = ContextFlagsBuilder::new().with_pts_dts_flags(0b10).build();

    assert!(OptionalFields::unmarshall(&data, context).is_none());
}

#[test]
fn test_pes_extension_data_validation() {
    let invalid_extension = PesExtensionData {
        size: 5,
        pes_extension_field_length: Some(10), // Greater than size
        pes_private_data_flag: false,
        pack_header_field_flag: false,
        program_packet_sequence_counter_flag: false,
        p_std_buffer_flag: false,
        pes_extension_flag_2: false,
        pes_private_data: None,
        pack_field_length: None,
        program_packet_sequence_counter: None,
        mpeg1_mpeg2_identifier: None,
        original_stuff_length: None,
        p_std_buffer_scale: None,
        p_std_buffer_size: None,
        stream_id_extension_flag: None,
        stream_id_extension: None,
        tref_extension_flag: None,
        tref: None,
    };
    assert!(!invalid_extension.validate());
}

#[test]
fn test_context_flags_builder() {
    let flags = ContextFlagsBuilder::new()
        .with_pts_dts_flags(0b11)
        .with_escr_flag(true)
        .with_es_rate_flag(true)
        .with_dsm_trick_mode_flag(false)
        .with_additional_copy_info_flag(true)
        .with_pes_crc_flag(false)
        .with_pes_extension_flag(true)
        .build();

    assert_eq!(flags.pts_dts_flags, 0b11);
    assert!(flags.escr_flag);
    assert!(flags.es_rate_flag);
    assert!(!flags.dsm_trick_mode_flag);
    assert!(flags.additional_copy_info_flag);
    assert!(!flags.pes_crc_flag);
    assert!(flags.pes_extension_flag);
}
