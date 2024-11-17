use super::*;

#[test]
fn test_table_id() {
    assert_eq!(TableId::from(0x00), TableId::ProgramAssociationSection);
    assert_eq!(TableId::from(0x01), TableId::ConditionalAccessSection);
    assert_eq!(TableId::from(0x02), TableId::TsProgramMapSection);
    assert_eq!(TableId::from(0x03), TableId::TsDescriptionSection);
    assert_eq!(
        TableId::from(0x04),
        TableId::IsoIec14496SceneDescriptionSection
    );
    assert_eq!(
        TableId::from(0x05),
        TableId::IsoIec14496ObjectDescriptorSection
    );
    assert_eq!(TableId::from(0x06), TableId::MetadataSection);
    assert_eq!(TableId::from(0x07), TableId::IpmpControlInformationSection);
    assert_eq!(TableId::from(0x08), TableId::IsoIec14496Section);
    assert_eq!(
        TableId::from(0x09),
        TableId::IsoIse23001_11GreenAccessUnitSection
    );
    assert_eq!(
        TableId::from(0x0A),
        TableId::IsoIse23001_10QualityAccessUnitSection
    );
    assert_eq!(
        TableId::from(0x0C),
        TableId::RecItuTH222_0IsoIec13818_1Reserved
    );
    assert_eq!(TableId::from(0x3F), TableId::DefinedInIsoIec13818_6);
    assert_eq!(TableId::from(0x41), TableId::UserPrivate);
    assert_eq!(TableId::from(0xFF), TableId::Forbidden);
}

#[test]
fn test_psi_header_eq() {
    let header1 = ProgramSpecificInformationHeader {
        table_id: 0,
        section_syntax_indicator: true,
        section_length: 49,
        version_number: 0,
        current_next_indicator: true,
        section_number: 0,
        last_section_number: 0,
    };

    let header2 = ProgramSpecificInformationHeader {
        table_id: 0,
        section_syntax_indicator: true,
        section_length: 49,
        version_number: 0,
        current_next_indicator: true,
        section_number: 0,
        last_section_number: 0,
    };

    assert_eq!(header1, header2);
}
