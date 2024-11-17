use super::*;

#[test]
fn test_stream_types() {
    assert_eq!(StreamTypes::from(0x00), StreamTypes::Reserved);
    assert_eq!(StreamTypes::from(0x01), StreamTypes::Video111722);
    assert_eq!(StreamTypes::from(0x02), StreamTypes::Video138182);
    assert_eq!(StreamTypes::from(0x03), StreamTypes::Audio111723);
    assert_eq!(StreamTypes::from(0x04), StreamTypes::Audio138183);
    assert_eq!(StreamTypes::from(0x05), StreamTypes::PrivateSections);
    assert_eq!(StreamTypes::from(0x06), StreamTypes::PESPackets);
    assert_eq!(StreamTypes::from(0x07), StreamTypes::MHEG);
    assert_eq!(StreamTypes::from(0x08), StreamTypes::DSMCC);
    assert_eq!(StreamTypes::from(0x09), StreamTypes::H2221);
    assert_eq!(StreamTypes::from(0x0A), StreamTypes::TypeA);
    assert_eq!(StreamTypes::from(0x0B), StreamTypes::TypeB);
    assert_eq!(StreamTypes::from(0x0C), StreamTypes::TypeC);
    assert_eq!(StreamTypes::from(0x0D), StreamTypes::TypeD);
    assert_eq!(StreamTypes::from(0x0E), StreamTypes::Auxiliary);
    assert_eq!(StreamTypes::from(0x0F), StreamTypes::AudioADTS);
    assert_eq!(StreamTypes::from(0x10), StreamTypes::Visual);
    assert_eq!(StreamTypes::from(0x11), StreamTypes::AudioLATM);
    assert_eq!(
        StreamTypes::from(0x12),
        StreamTypes::SLpacketizedstreamorFlexMuxstreamPESPacketized
    );
    assert_eq!(
        StreamTypes::from(0x13),
        StreamTypes::SLpacketizedstreamorFlexMuxstreamSectionPacketized
    );
    assert_eq!(StreamTypes::from(0x14), StreamTypes::Download);
    assert_eq!(StreamTypes::from(0x15), StreamTypes::MetadataPES);
    assert_eq!(StreamTypes::from(0x16), StreamTypes::MetadataSections);
    assert_eq!(StreamTypes::from(0x17), StreamTypes::MetadataCarousel);
    assert_eq!(StreamTypes::from(0x18), StreamTypes::MetadataObject);
    assert_eq!(StreamTypes::from(0x19), StreamTypes::MetadataDownload);
    assert_eq!(StreamTypes::from(0x1A), StreamTypes::IPMP);
    assert_eq!(StreamTypes::from(0x1B), StreamTypes::VideoH264);
    assert_eq!(StreamTypes::from(0x1C), StreamTypes::RawAudio);
    assert_eq!(StreamTypes::from(0x1D), StreamTypes::Text);
    assert_eq!(StreamTypes::from(0x1E), StreamTypes::AuxiliaryVideo);
    assert_eq!(StreamTypes::from(0x1F), StreamTypes::VideoSVC);
    assert_eq!(StreamTypes::from(0x20), StreamTypes::VideoMVC);
    assert_eq!(StreamTypes::from(0x21), StreamTypes::ConformingVideoStream);
    assert_eq!(StreamTypes::from(0x22), StreamTypes::Video3DH264);
    assert_eq!(StreamTypes::from(0x23), StreamTypes::VideoHEVC);
    assert_eq!(StreamTypes::from(0x24), StreamTypes::VideoHEVCTemporal);
    assert_eq!(StreamTypes::from(0x25), StreamTypes::VideoMVCD);
    assert_eq!(StreamTypes::from(0x26), StreamTypes::Timeline);
    assert_eq!(StreamTypes::from(0x27), StreamTypes::VideoHEVCEnhanceG);
    assert_eq!(StreamTypes::from(0x28), StreamTypes::VideoHEVCEnhanceGTemp);
    assert_eq!(StreamTypes::from(0x29), StreamTypes::VideoHEVCEnhanceH);
    assert_eq!(StreamTypes::from(0x2A), StreamTypes::VideoHEVCEnhanceHTemp);
    assert_eq!(StreamTypes::from(0x2B), StreamTypes::GreenUnits);
    assert_eq!(StreamTypes::from(0x2C), StreamTypes::AudioMHASMain);
    assert_eq!(StreamTypes::from(0x2D), StreamTypes::AudioMHASAux);
    assert_eq!(StreamTypes::from(0x2E), StreamTypes::QualityUnits);
    assert_eq!(StreamTypes::from(0x6F), StreamTypes::Reserved138181);
    assert_eq!(StreamTypes::from(0x7F), StreamTypes::IPMPStream);
    assert_eq!(StreamTypes::from(0x80), StreamTypes::UserPrivate);
}

#[test]
fn test_edge_cases() {
    // Test values above 0x80
    assert_eq!(StreamTypes::from(0x81), StreamTypes::UserPrivate);
    assert_eq!(StreamTypes::from(0xFF), StreamTypes::UserPrivate);

    // Test values between reserved range
    assert_eq!(StreamTypes::from(0x31), StreamTypes::Reserved138181);
    assert_eq!(StreamTypes::from(0x7E), StreamTypes::Reserved138181);
}
#[test]
fn test_display_for_stream_types() {
    assert_eq!(format!("{}", StreamTypes::Reserved), "Reserved");
    assert_eq!(format!("{}", StreamTypes::Video111722), "MPEG-1 Video");
    assert_eq!(format!("{}", StreamTypes::Video138182), "MPEG-2 Video");
    assert_eq!(format!("{}", StreamTypes::Audio111723), "MPEG-1 Audio");
    assert_eq!(format!("{}", StreamTypes::Audio138183), "MPEG-2 Audio");
    assert_eq!(
        format!("{}", StreamTypes::PrivateSections),
        "Private Sections"
    );
    assert_eq!(format!("{}", StreamTypes::PESPackets), "PES Packets");
    assert_eq!(format!("{}", StreamTypes::MHEG), "MHEG");
    assert_eq!(format!("{}", StreamTypes::DSMCC), "DSM-CC");
    assert_eq!(format!("{}", StreamTypes::H2221), "H.222.1");
    assert_eq!(format!("{}", StreamTypes::TypeA), "Type A");
    assert_eq!(format!("{}", StreamTypes::TypeB), "Type B");
    assert_eq!(format!("{}", StreamTypes::TypeC), "Type C");
    assert_eq!(format!("{}", StreamTypes::TypeD), "Type D");
    assert_eq!(format!("{}", StreamTypes::Auxiliary), "Auxiliary");
    assert_eq!(format!("{}", StreamTypes::AudioADTS), "ADTS Audio");
    assert_eq!(format!("{}", StreamTypes::Visual), "MPEG-4 Visual");
    assert_eq!(format!("{}", StreamTypes::AudioLATM), "LATM Audio");
    assert_eq!(
        format!(
            "{}",
            StreamTypes::SLpacketizedstreamorFlexMuxstreamPESPacketized
        ),
        "SL/FlexMux in PES"
    );
    assert_eq!(
        format!(
            "{}",
            StreamTypes::SLpacketizedstreamorFlexMuxstreamSectionPacketized
        ),
        "SL/FlexMux in Sections"
    );
    assert_eq!(
        format!("{}", StreamTypes::Download),
        "Synchronized Download Protocol"
    );
    assert_eq!(format!("{}", StreamTypes::MetadataPES), "Metadata PES");
    assert_eq!(
        format!("{}", StreamTypes::MetadataSections),
        "Metadata Sections"
    );
    assert_eq!(
        format!("{}", StreamTypes::MetadataCarousel),
        "Metadata Carousel"
    );
    assert_eq!(
        format!("{}", StreamTypes::MetadataObject),
        "Metadata Object"
    );
    assert_eq!(
        format!("{}", StreamTypes::MetadataDownload),
        "Metadata Synchronized Download Protocol"
    );
    assert_eq!(format!("{}", StreamTypes::IPMP), "IPMP");
    assert_eq!(format!("{}", StreamTypes::VideoH264), "H.264 Video");
    assert_eq!(format!("{}", StreamTypes::RawAudio), "Raw Audio");
    assert_eq!(format!("{}", StreamTypes::Text), "Text");
    assert_eq!(
        format!("{}", StreamTypes::AuxiliaryVideo),
        "Auxiliary Video"
    );
    assert_eq!(format!("{}", StreamTypes::VideoSVC), "SVC Video");
    assert_eq!(format!("{}", StreamTypes::VideoMVC), "MVC Video");
    assert_eq!(
        format!("{}", StreamTypes::ConformingVideoStream),
        "Conforming Video Stream"
    );
    assert_eq!(format!("{}", StreamTypes::Video3DH264), "3D H.264 Video");
    assert_eq!(format!("{}", StreamTypes::VideoHEVC), "HEVC Video");
    assert_eq!(
        format!("{}", StreamTypes::VideoHEVCTemporal),
        "HEVC Temporal Video"
    );
    assert_eq!(format!("{}", StreamTypes::VideoMVCD), "MVCD  Video");
    assert_eq!(format!("{}", StreamTypes::Timeline), "Timeline");
    assert_eq!(
        format!("{}", StreamTypes::VideoHEVCEnhanceG),
        "HEVC Enhancement G  Video"
    );
    assert_eq!(
        format!("{}", StreamTypes::VideoHEVCEnhanceGTemp),
        "HEVC Temporal G  Video"
    );
    assert_eq!(
        format!("{}", StreamTypes::VideoHEVCEnhanceH),
        "HEVC Enhancement H  Video"
    );
    assert_eq!(
        format!("{}", StreamTypes::VideoHEVCEnhanceHTemp),
        "HEVC Temporal H  Video"
    );
    assert_eq!(format!("{}", StreamTypes::GreenUnits), "Green Units");
    assert_eq!(format!("{}", StreamTypes::AudioMHASMain), "MHAS Main Audio");
    assert_eq!(format!("{}", StreamTypes::AudioMHASAux), "MHAS Aux Audio");
    assert_eq!(format!("{}", StreamTypes::QualityUnits), "Quality Units");
    assert_eq!(format!("{}", StreamTypes::Reserved138181), "Reserved");
    assert_eq!(format!("{}", StreamTypes::IPMPStream), "IPMP Stream");
    assert_eq!(format!("{}", StreamTypes::UserPrivate), "User Private");
}
