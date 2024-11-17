use super::*;

#[test]
fn test_optional_fields_unmarshall_pts_dts() {
    let data = [0b00001011, 0b10110011, 0b11101001, 0b10110011, 0b10000011];
    let result = OptionalFields::unmarshall_pts_dts(&data);
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
#[test]
fn test_optional_fields_unmarshall_escr() {
    let data = [
        0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
    ];
    let result = OptionalFields::unmarshall_escr(&data);
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

#[test]
fn test_optional_fields_unmarshall_es_rate() {
    let data = [0b11111111, 0b11111111, 0b11111111];
    let result = OptionalFields::unmarshall_es_rate(&data);
    assert_eq!(result, Ok(4_194_303));
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
    let result = PesExtensionData::unmarshall_tref(&data);
    assert_eq!(result, Ok(8_589_934_591));
}

#[test]
fn test_unmarshall_tref_invalid() {
    let data = [0b11111111, 0b11111111, 0b11111111, 0b11111111];
    let result = PesExtensionData::unmarshall_tref(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_unmarshall_tref_invalid_first_marker_bit() {
    let data = [0b11111110, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
    let result = PesExtensionData::unmarshall_tref(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_unmarshall_tref_invalid_second_marker_bit() {
    let data = [0b11111111, 0b11111111, 0b11111110, 0b11111111, 0b11111111];
    let result = PesExtensionData::unmarshall_tref(&data);
    assert_eq!(result, Err(()));
}

#[test]
fn test_unmarshall_tref_invalid_third_marker_bit() {
    let data = [0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111110];
    let result = PesExtensionData::unmarshall_tref(&data);
    assert_eq!(result, Err(()));
}
