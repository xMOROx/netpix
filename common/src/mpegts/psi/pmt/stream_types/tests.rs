use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_stream_types() {
    assert_eq!(StreamType::from(0x00), StreamType::Reserved);
    assert_eq!(StreamType::from(0x01), StreamType::Video111722);
    assert_eq!(StreamType::from(0x02), StreamType::Video138182);
    assert_eq!(StreamType::from(0x03), StreamType::Audio111723);
    assert_eq!(StreamType::from(0x04), StreamType::Audio138183);
    assert_eq!(StreamType::from(0x05), StreamType::PrivateSections);
    assert_eq!(StreamType::from(0x06), StreamType::PESPackets);
    assert_eq!(StreamType::from(0x07), StreamType::MHEG);
    assert_eq!(StreamType::from(0x08), StreamType::DSMCC);
    assert_eq!(StreamType::from(0x09), StreamType::H2221);
    assert_eq!(StreamType::from(0x0A), StreamType::TypeA);
    assert_eq!(StreamType::from(0x0B), StreamType::TypeB);
    assert_eq!(StreamType::from(0x0C), StreamType::TypeC);
    assert_eq!(StreamType::from(0x0D), StreamType::TypeD);
    assert_eq!(StreamType::from(0x0E), StreamType::Auxiliary);
    assert_eq!(StreamType::from(0x0F), StreamType::AudioADTS);
    assert_eq!(StreamType::from(0x10), StreamType::Visual);
    assert_eq!(StreamType::from(0x11), StreamType::AudioLATM);
    assert_eq!(
        StreamType::from(0x12),
        StreamType::SLpacketizedstreamorFlexMuxstreamPESPacketized
    );
    assert_eq!(
        StreamType::from(0x13),
        StreamType::SLpacketizedstreamorFlexMuxstreamSectionPacketized
    );
    assert_eq!(StreamType::from(0x14), StreamType::Download);
    assert_eq!(StreamType::from(0x15), StreamType::MetadataPES);
    assert_eq!(StreamType::from(0x16), StreamType::MetadataSections);
    assert_eq!(StreamType::from(0x17), StreamType::MetadataCarousel);
    assert_eq!(StreamType::from(0x18), StreamType::MetadataObject);
    assert_eq!(StreamType::from(0x19), StreamType::MetadataDownload);
    assert_eq!(StreamType::from(0x1A), StreamType::IPMP);
    assert_eq!(StreamType::from(0x1B), StreamType::VideoH264);
    assert_eq!(StreamType::from(0x1C), StreamType::RawAudio);
    assert_eq!(StreamType::from(0x1D), StreamType::Text);
    assert_eq!(StreamType::from(0x1E), StreamType::AuxiliaryVideo);
    assert_eq!(StreamType::from(0x1F), StreamType::VideoSVC);
    assert_eq!(StreamType::from(0x20), StreamType::VideoMVC);
    assert_eq!(StreamType::from(0x21), StreamType::ConformingVideoStream);
    assert_eq!(StreamType::from(0x22), StreamType::Video3DH264);
    assert_eq!(StreamType::from(0x23), StreamType::VideoHEVC);
    assert_eq!(StreamType::from(0x24), StreamType::VideoHEVCTemporal);
    assert_eq!(StreamType::from(0x25), StreamType::VideoMVCD);
    assert_eq!(StreamType::from(0x26), StreamType::Timeline);
    assert_eq!(StreamType::from(0x27), StreamType::VideoHEVCEnhanceG);
    assert_eq!(StreamType::from(0x28), StreamType::VideoHEVCEnhanceGTemp);
    assert_eq!(StreamType::from(0x29), StreamType::VideoHEVCEnhanceH);
    assert_eq!(StreamType::from(0x2A), StreamType::VideoHEVCEnhanceHTemp);
    assert_eq!(StreamType::from(0x2B), StreamType::GreenUnits);
    assert_eq!(StreamType::from(0x2C), StreamType::AudioMHASMain);
    assert_eq!(StreamType::from(0x2D), StreamType::AudioMHASAux);
    assert_eq!(StreamType::from(0x2E), StreamType::QualityUnits);
    assert_eq!(StreamType::from(0x6F), StreamType::Reserved138181);
    assert_eq!(StreamType::from(0x7F), StreamType::IPMPStream);
    assert_eq!(StreamType::from(0x80), StreamType::UserPrivate);
}

#[test]
fn test_edge_cases() {
    // Test values above 0x80
    assert_eq!(StreamType::from(0x81), StreamType::UserPrivate);
    assert_eq!(StreamType::from(0xFF), StreamType::UserPrivate);

    // Test values between reserved range
    assert_eq!(StreamType::from(0x31), StreamType::Reserved138181);
    assert_eq!(StreamType::from(0x7E), StreamType::Reserved138181);
}
#[test]
fn test_display_for_stream_types() {
    assert_eq!(format!("{}", StreamType::Reserved), "Reserved");
    assert_eq!(format!("{}", StreamType::Video111722), "MPEG-1 Video");
    assert_eq!(format!("{}", StreamType::Video138182), "MPEG-2 Video");
    assert_eq!(format!("{}", StreamType::Audio111723), "MPEG-1 Audio");
    assert_eq!(format!("{}", StreamType::Audio138183), "MPEG-2 Audio");
    assert_eq!(
        format!("{}", StreamType::PrivateSections),
        "Private Sections"
    );
    assert_eq!(format!("{}", StreamType::PESPackets), "PES Packets");
    assert_eq!(format!("{}", StreamType::MHEG), "MHEG");
    assert_eq!(format!("{}", StreamType::DSMCC), "DSM-CC");
    assert_eq!(format!("{}", StreamType::H2221), "H.222.1");
    assert_eq!(format!("{}", StreamType::TypeA), "Type A");
    assert_eq!(format!("{}", StreamType::TypeB), "Type B");
    assert_eq!(format!("{}", StreamType::TypeC), "Type C");
    assert_eq!(format!("{}", StreamType::TypeD), "Type D");
    assert_eq!(format!("{}", StreamType::Auxiliary), "Auxiliary");
    assert_eq!(format!("{}", StreamType::AudioADTS), "ADTS Audio");
    assert_eq!(format!("{}", StreamType::Visual), "MPEG-4 Visual");
    assert_eq!(format!("{}", StreamType::AudioLATM), "LATM Audio");
    assert_eq!(
        format!(
            "{}",
            StreamType::SLpacketizedstreamorFlexMuxstreamPESPacketized
        ),
        "SL/FlexMux in PES"
    );
    assert_eq!(
        format!(
            "{}",
            StreamType::SLpacketizedstreamorFlexMuxstreamSectionPacketized
        ),
        "SL/FlexMux in Sections"
    );
    assert_eq!(
        format!("{}", StreamType::Download),
        "Synchronized Download Protocol"
    );
    assert_eq!(format!("{}", StreamType::MetadataPES), "Metadata PES");
    assert_eq!(
        format!("{}", StreamType::MetadataSections),
        "Metadata Sections"
    );
    assert_eq!(
        format!("{}", StreamType::MetadataCarousel),
        "Metadata Carousel"
    );
    assert_eq!(format!("{}", StreamType::MetadataObject), "Metadata Object");
    assert_eq!(
        format!("{}", StreamType::MetadataDownload),
        "Metadata Synchronized Download Protocol"
    );
    assert_eq!(format!("{}", StreamType::IPMP), "IPMP");
    assert_eq!(format!("{}", StreamType::VideoH264), "H.264 Video");
    assert_eq!(format!("{}", StreamType::RawAudio), "Raw Audio");
    assert_eq!(format!("{}", StreamType::Text), "Text");
    assert_eq!(format!("{}", StreamType::AuxiliaryVideo), "Auxiliary Video");
    assert_eq!(format!("{}", StreamType::VideoSVC), "SVC Video");
    assert_eq!(format!("{}", StreamType::VideoMVC), "MVC Video");
    assert_eq!(
        format!("{}", StreamType::ConformingVideoStream),
        "Conforming Video Stream"
    );
    assert_eq!(format!("{}", StreamType::Video3DH264), "3D H.264 Video");
    assert_eq!(format!("{}", StreamType::VideoHEVC), "HEVC Video");
    assert_eq!(
        format!("{}", StreamType::VideoHEVCTemporal),
        "HEVC Temporal Video"
    );
    assert_eq!(format!("{}", StreamType::VideoMVCD), "MVCD  Video");
    assert_eq!(format!("{}", StreamType::Timeline), "Timeline");
    assert_eq!(
        format!("{}", StreamType::VideoHEVCEnhanceG),
        "HEVC Enhancement G  Video"
    );
    assert_eq!(
        format!("{}", StreamType::VideoHEVCEnhanceGTemp),
        "HEVC Temporal G  Video"
    );
    assert_eq!(
        format!("{}", StreamType::VideoHEVCEnhanceH),
        "HEVC Enhancement H  Video"
    );
    assert_eq!(
        format!("{}", StreamType::VideoHEVCEnhanceHTemp),
        "HEVC Temporal H  Video"
    );
    assert_eq!(format!("{}", StreamType::GreenUnits), "Green Units");
    assert_eq!(format!("{}", StreamType::AudioMHASMain), "MHAS Main Audio");
    assert_eq!(format!("{}", StreamType::AudioMHASAux), "MHAS Aux Audio");
    assert_eq!(format!("{}", StreamType::QualityUnits), "Quality Units");
    assert_eq!(format!("{}", StreamType::Reserved138181), "Reserved");
    assert_eq!(format!("{}", StreamType::IPMPStream), "IPMP Stream");
    assert_eq!(format!("{}", StreamType::UserPrivate), "User Private");
}
