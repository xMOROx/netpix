use super::*;

#[test]
fn test_stream_types() {
    assert_eq!(StreamTypes::from(0x00), StreamTypes::ItuTIsoIecReserved);
    assert_eq!(StreamTypes::from(0x01), StreamTypes::IsoIec111722Video);
    assert_eq!(
        StreamTypes::from(0x02),
        StreamTypes::RecItuTH262OrIsoIec138182Video
    );
    assert_eq!(StreamTypes::from(0x03), StreamTypes::IsoIec111723Audio);
    assert_eq!(StreamTypes::from(0x04), StreamTypes::IsoIec138183Audio);
    assert_eq!(
        StreamTypes::from(0x05),
        StreamTypes::RecItuTH2220OrIsoIec138181PrivateSections
    );
    assert_eq!(
        StreamTypes::from(0x06),
        StreamTypes::RecItuTH2220OrIsoIec138181PESPackets
    );
    assert_eq!(StreamTypes::from(0x07), StreamTypes::IsoIec13522MHEG);
    assert_eq!(
        StreamTypes::from(0x08),
        StreamTypes::RecItuTH2220OrIsoIec138181AnnexADSMCC
    );
    assert_eq!(StreamTypes::from(0x09), StreamTypes::RecItuTH2221);
    assert_eq!(StreamTypes::from(0x0A), StreamTypes::IsoIec138186TypeA);
    assert_eq!(StreamTypes::from(0x0B), StreamTypes::IsoIec138186TypeB);
    assert_eq!(StreamTypes::from(0x0C), StreamTypes::IsoIec138186TypeC);
    assert_eq!(StreamTypes::from(0x0D), StreamTypes::IsoIec138186TypeD);
    assert_eq!(
        StreamTypes::from(0x0E),
        StreamTypes::RecItuTH2220OrIsoIec138181Auxiliary
    );
    assert_eq!(
        StreamTypes::from(0x0F),
        StreamTypes::IsoIec138187AudioWithADTSTransportSyntax
    );
    assert_eq!(StreamTypes::from(0x10), StreamTypes::IsoIec144962Visual);
    assert_eq!(
        StreamTypes::from(0x11),
        StreamTypes::IsoIec144963AudioWithLATMTransportSyntax
    );
    assert_eq!(
        StreamTypes::from(0x12),
        StreamTypes::IsoIec144961SLPacketizedStreamOrFlexMuxStreamInPESPackets
    );
    assert_eq!(
        StreamTypes::from(0x13),
        StreamTypes::IsoIec144961SLPacketizedStreamOrFlexMuxStreamInIsoIec14496Sections
    );
    assert_eq!(
        StreamTypes::from(0x14),
        StreamTypes::IsoIec138186SynchronizedDownloadProtocol
    );
    assert_eq!(StreamTypes::from(0x15), StreamTypes::MetadataInPESPackets);
    assert_eq!(
        StreamTypes::from(0x16),
        StreamTypes::MetadataInMetadataSections
    );
    assert_eq!(
        StreamTypes::from(0x17),
        StreamTypes::MetadataInIsoIec138186DataCarousel
    );
    assert_eq!(
        StreamTypes::from(0x18),
        StreamTypes::MetadataInIsoIec138186ObjectCarousel
    );
    assert_eq!(
        StreamTypes::from(0x19),
        StreamTypes::MetadataInIsoIec138186SynchronizedDownloadProtocol
    );
    assert_eq!(
        StreamTypes::from(0x1A),
        StreamTypes::IpmpstreamDefinedInIsoIec1381811mpeg2
    );
    assert_eq!(
        StreamTypes::from(0x1B),
        StreamTypes::AVCVideoStreamAsDefinedInItuTH264OrIsoIec1449610Video
    );
    assert_eq!(
        StreamTypes::from(0x1C),
        StreamTypes::IsoIec144963AudioWithoutAdditionalTransportSyntax
    );
    assert_eq!(StreamTypes::from(0x1D), StreamTypes::IsoIec1449617Text);
    assert_eq!(
        StreamTypes::from(0x1E),
        StreamTypes::AuxiliaryVideoStreamAsDefinedInIsoIec230023
    );
    assert_eq!(
        StreamTypes::from(0x1F),
        StreamTypes::SVCVideoStreamAsDefinedInIsoIec1449610
    );
    assert_eq!(
        StreamTypes::from(0x20),
        StreamTypes::MVCVideoStreamAsDefinedInIsoIec1449610
    );
    assert_eq!(
        StreamTypes::from(0x21),
        StreamTypes::VideoStreamConformingToOneOrMoreProfilesAsDefinedInRecItuTT800OrIsoIec154441
    );
    assert_eq!(StreamTypes::from(0x22), StreamTypes::AdditionalViewRecItuTH262OrIsoIec138182VideoStreamForServiceCompatibleStereoscopic3DServices);
    assert_eq!(
        StreamTypes::from(0x23),
        StreamTypes::AdditionalViewRecItuTH264OrIsoIec1449610VideoStreamForServiceCompatible
    );
    assert_eq!(
        StreamTypes::from(0x24),
        StreamTypes::RecItuTH265OrIsoIec230082VideoStreamOrAnHEVCTemporalVideoSubBitstream
    );
    assert_eq!(StreamTypes::from(0x25), StreamTypes::HEVCTemporalVideoSubsetOfAnHEVCVideoStreamConformingToOneOrMoreProfilesDefinedInAnnexAOfRecItuTH265OrIsoIec230082);
    assert_eq!(StreamTypes::from(0x26), StreamTypes::MVCDVideoSubBitstreamOfAnAVCVideoStreamConformingToOneOrMoreProfilesDefinedInAnnexIOfRecItuTH264OrIsoIec1449610);
    assert_eq!(
        StreamTypes::from(0x27),
        StreamTypes::TimelineAndExternalMediaInformationStream
    );
    assert_eq!(StreamTypes::from(0x28), StreamTypes::HEVCEnhancementSubPartitionWhichIncludesTemporalId0OfAnHEVCVideoStreamWhereAllNALsUnitsContainedInTheStreamConformToOneOrMoreProfilesDefinedInAnnexGOfRecItuTH265OrIsoIec230082);
    assert_eq!(StreamTypes::from(0x29), StreamTypes::HEVCTemporalEnhancementSubPartitionOfAnHEVCVideoStreamWhereAllNALUnitsContainedInTheStreamConformToOneOrMoreProfilesDefinedInAnnexGOfRecItuTH265OrIsoIec230082);
    assert_eq!(StreamTypes::from(0x2A), StreamTypes::HEVCEnhancementSubPartitionWhichIncludesTemporalId0OfAnHEVCVideoStreamWhereAllNALUnitsContainedInTheStreamConformToOneOrMoreProfilesDefinedInAnnexHOfRecItuTH265OrIsoIec230082);
    assert_eq!(StreamTypes::from(0x2B), StreamTypes::HEVCTemporalEnhancementSubPartitionOfAnHEVCVideoStreamWhereAllNALUnitsContainedInTheStreamConformToOneOrMoreProfilesDefinedInAnnexHOfRecItuTH265OrIsoIec230082);
    assert_eq!(
        StreamTypes::from(0x2C),
        StreamTypes::GreenAccessUnitsCarriedInMPEG2Sections
    );
    assert_eq!(
        StreamTypes::from(0x2D),
        StreamTypes::IsoIec230083AudioWithMHASTransportSyntaxMainStream
    );
    assert_eq!(
        StreamTypes::from(0x2E),
        StreamTypes::IsoIec230083AudioWithMHASTransportSyntaxAuxiliaryStream
    );
    assert_eq!(
        StreamTypes::from(0x2F),
        StreamTypes::QualityAccessUnitsCarriedInSections
    );
    assert_eq!(
        StreamTypes::from(0x30),
        StreamTypes::RecItuTH2220OrIsoIec138181Reserved
    );
    assert_eq!(StreamTypes::from(0x7F), StreamTypes::IPMPStream);
    assert_eq!(StreamTypes::from(0x80), StreamTypes::UserPrivate);
}

#[test]
fn test_display_for_stream_types() {
    assert_eq!(
        format!("{}", StreamTypes::ItuTIsoIecReserved),
        "ITU-T | ISO/IEC Reserved"
    );
    assert_eq!(
        format!("{}", StreamTypes::IsoIec111722Video),
        "ISO/IEC 11172-2 Video"
    );
    assert_eq!(
        format!("{}", StreamTypes::RecItuTH262OrIsoIec138182Video),
        "Rec. ITU-T H.262 | ISO/IEC 13818-2 Video"
    );
    assert_eq!(
        format!("{}", StreamTypes::IsoIec111723Audio),
        "ISO/IEC 11172-3 Audio"
    );
    assert_eq!(
        format!("{}", StreamTypes::IsoIec138183Audio),
        "ISO/IEC 13818-3 Audio"
    );
}
