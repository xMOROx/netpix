use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, Default)]
pub enum DescriptorTag {
    VideoStreamDescriptorTag,
    AudioStreamDescriptorTag,
    HierarchyDescriptorTag,
    RegistrationDescriptorTag,
    DataStreamAlignmentDescriptorTag,
    TargetBackgroundGridDescriptorTag,
    VideoWindowDescriptorTag,
    CaDescriptorTag,
    Iso639LanguageDescriptorTag,
    SystemClockDescriptorTag,
    MultiplexBufferUtilizationDescriptorTag,
    CopyrightDescriptorTag,
    MaximumBitrateDescriptorTag,
    PrivateDataIndicatorDescriptorTag,
    SmoothingBufferDescriptorTag,
    StdDescriptorTag,
    IbpDescriptorTag,
    Mpeg4VideoDescriptorTag,
    Mpeg4AudioDescriptorTag,
    IodDescriptorTag,
    SlDescriptorTag,
    FmcDescriptorTag,
    ExternalEsIdDescriptorTag,
    MuxCodeDescriptorTag,
    FmxBufferSizeDescriptorTag,
    MultiplexbufferDescriptorTag,
    ContentLabelingDescriptorTag,
    MetadataPointerDescriptorTag,
    MetadataDescriptorTag,
    MetadataStdDescriptorTag,
    AvcVideoDescriptorTag,
    AvcTimingAndHrdDescriptorTag,
    Mpeg2AacAudioDescriptorTag,
    FlexMuxTimingDescriptorTag,
    Mpeg4TextDescriptorTag,
    Mpeg4AudioExtensionDescriptorTag,
    AuxiliaryVideoStreamDescriptorTag,
    SvcExtensionDescriptorTag,
    MvcExtensionDescriptorTag,
    J2kVideoDescriptorTag,
    MvcOperationPointDescriptorTag,
    Mpeg2StereoscopicVideoFormatDescriptorTag,
    StereoscopicProgramInfoDescriptorTag,
    StereoscopicVideoInfoDescriptorTag,
    TransportProfileDescriptorTag,
    HevcVideoDescriptorTag,
    ExtensionDescriptorTag,
    UserPrivate,
    #[default]
    Unknown,
}

impl DescriptorTag {
    pub fn to_u8(&self) -> u8 {
        match self {
            DescriptorTag::VideoStreamDescriptorTag => 0x02,
            DescriptorTag::AudioStreamDescriptorTag => 0x03,
            DescriptorTag::HierarchyDescriptorTag => 0x04,
            DescriptorTag::RegistrationDescriptorTag => 0x05,
            DescriptorTag::DataStreamAlignmentDescriptorTag => 0x06,
            DescriptorTag::TargetBackgroundGridDescriptorTag => 0x07,
            DescriptorTag::VideoWindowDescriptorTag => 0x08,
            DescriptorTag::CaDescriptorTag => 0x09,
            DescriptorTag::Iso639LanguageDescriptorTag => 0x0A,
            DescriptorTag::SystemClockDescriptorTag => 0x0B,
            DescriptorTag::MultiplexBufferUtilizationDescriptorTag => 0x0C,
            DescriptorTag::CopyrightDescriptorTag => 0x0D,
            DescriptorTag::MaximumBitrateDescriptorTag => 0x0E,
            DescriptorTag::PrivateDataIndicatorDescriptorTag => 0x0F,
            DescriptorTag::SmoothingBufferDescriptorTag => 0x10,
            DescriptorTag::StdDescriptorTag => 0x11,
            DescriptorTag::IbpDescriptorTag => 0x12,
            DescriptorTag::Mpeg4VideoDescriptorTag => 0x1B,
            DescriptorTag::Mpeg4AudioDescriptorTag => 0x1C,
            DescriptorTag::IodDescriptorTag => 0x1D,
            DescriptorTag::SlDescriptorTag => 0x1E,
            DescriptorTag::FmcDescriptorTag => 0x1F,
            DescriptorTag::ExternalEsIdDescriptorTag => 0x20,
            DescriptorTag::MuxCodeDescriptorTag => 0x21,
            DescriptorTag::FmxBufferSizeDescriptorTag => 0x22,
            DescriptorTag::MultiplexbufferDescriptorTag => 0x23,
            DescriptorTag::ContentLabelingDescriptorTag => 0x24,
            DescriptorTag::MetadataPointerDescriptorTag => 0x25,
            DescriptorTag::MetadataDescriptorTag => 0x26,
            DescriptorTag::MetadataStdDescriptorTag => 0x27,
            DescriptorTag::AvcVideoDescriptorTag => 0x28,
            DescriptorTag::AvcTimingAndHrdDescriptorTag => 0x2A,
            DescriptorTag::Mpeg2AacAudioDescriptorTag => 0x2B,
            DescriptorTag::FlexMuxTimingDescriptorTag => 0x2C,
            DescriptorTag::Mpeg4TextDescriptorTag => 0x2D,
            DescriptorTag::Mpeg4AudioExtensionDescriptorTag => 0x2E,
            DescriptorTag::AuxiliaryVideoStreamDescriptorTag => 0x2F,
            DescriptorTag::SvcExtensionDescriptorTag => 0x30,
            DescriptorTag::MvcExtensionDescriptorTag => 0x31,
            DescriptorTag::J2kVideoDescriptorTag => 0x32,
            DescriptorTag::MvcOperationPointDescriptorTag => 0x33,
            DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptorTag => 0x34,
            DescriptorTag::StereoscopicProgramInfoDescriptorTag => 0x35,
            DescriptorTag::StereoscopicVideoInfoDescriptorTag => 0x36,
            DescriptorTag::TransportProfileDescriptorTag => 0x37,
            DescriptorTag::HevcVideoDescriptorTag => 0x38,
            DescriptorTag::ExtensionDescriptorTag => 0x3F,
            DescriptorTag::UserPrivate => 0x40,
            DescriptorTag::Unknown => 0x00,
        }
    }
}

impl From<u8> for DescriptorTag {
    fn from(value: u8) -> Self {
        match value {
            0x02 => DescriptorTag::VideoStreamDescriptorTag,
            0x03 => DescriptorTag::AudioStreamDescriptorTag,
            0x04 => DescriptorTag::HierarchyDescriptorTag,
            0x05 => DescriptorTag::RegistrationDescriptorTag,
            0x06 => DescriptorTag::DataStreamAlignmentDescriptorTag,
            0x07 => DescriptorTag::TargetBackgroundGridDescriptorTag,
            0x08 => DescriptorTag::VideoWindowDescriptorTag,
            0x09 => DescriptorTag::CaDescriptorTag,
            0x0A => DescriptorTag::Iso639LanguageDescriptorTag,
            0x0B => DescriptorTag::SystemClockDescriptorTag,
            0x0C => DescriptorTag::MultiplexBufferUtilizationDescriptorTag,
            0x0D => DescriptorTag::CopyrightDescriptorTag,
            0x0E => DescriptorTag::MaximumBitrateDescriptorTag,
            0x0F => DescriptorTag::PrivateDataIndicatorDescriptorTag,
            0x10 => DescriptorTag::SmoothingBufferDescriptorTag,
            0x11 => DescriptorTag::StdDescriptorTag,
            0x12 => DescriptorTag::IbpDescriptorTag,
            0x1B => DescriptorTag::Mpeg4VideoDescriptorTag,
            0x1C => DescriptorTag::Mpeg4AudioDescriptorTag,
            0x1D => DescriptorTag::IodDescriptorTag,
            0x1E => DescriptorTag::SlDescriptorTag,
            0x1F => DescriptorTag::FmcDescriptorTag,
            0x20 => DescriptorTag::ExternalEsIdDescriptorTag,
            0x21 => DescriptorTag::MuxCodeDescriptorTag,
            0x22 => DescriptorTag::FmxBufferSizeDescriptorTag,
            0x23 => DescriptorTag::MultiplexbufferDescriptorTag,
            0x24 => DescriptorTag::ContentLabelingDescriptorTag,
            0x25 => DescriptorTag::MetadataPointerDescriptorTag,
            0x26 => DescriptorTag::MetadataDescriptorTag,
            0x27 => DescriptorTag::MetadataStdDescriptorTag,
            0x28 => DescriptorTag::AvcVideoDescriptorTag,
            0x2A => DescriptorTag::AvcTimingAndHrdDescriptorTag,
            0x2B => DescriptorTag::Mpeg2AacAudioDescriptorTag,
            0x2C => DescriptorTag::FlexMuxTimingDescriptorTag,
            0x2D => DescriptorTag::Mpeg4TextDescriptorTag,
            0x2E => DescriptorTag::Mpeg4AudioExtensionDescriptorTag,
            0x2F => DescriptorTag::AuxiliaryVideoStreamDescriptorTag,
            0x30 => DescriptorTag::SvcExtensionDescriptorTag,
            0x31 => DescriptorTag::MvcExtensionDescriptorTag,
            0x32 => DescriptorTag::J2kVideoDescriptorTag,
            0x33 => DescriptorTag::MvcOperationPointDescriptorTag,
            0x34 => DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptorTag,
            0x35 => DescriptorTag::StereoscopicProgramInfoDescriptorTag,
            0x36 => DescriptorTag::StereoscopicVideoInfoDescriptorTag,
            0x37 => DescriptorTag::TransportProfileDescriptorTag,
            0x38 => DescriptorTag::HevcVideoDescriptorTag,
            0x3F => DescriptorTag::ExtensionDescriptorTag,
            0x40..=0xFF => DescriptorTag::UserPrivate,
            _ => DescriptorTag::Unknown,
        }
    }
}

impl PartialEq for DescriptorTag {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (
                DescriptorTag::VideoStreamDescriptorTag,
                DescriptorTag::VideoStreamDescriptorTag
            ) | (
                DescriptorTag::AudioStreamDescriptorTag,
                DescriptorTag::AudioStreamDescriptorTag
            ) | (
                DescriptorTag::HierarchyDescriptorTag,
                DescriptorTag::HierarchyDescriptorTag
            ) | (
                DescriptorTag::RegistrationDescriptorTag,
                DescriptorTag::RegistrationDescriptorTag
            ) | (
                DescriptorTag::DataStreamAlignmentDescriptorTag,
                DescriptorTag::DataStreamAlignmentDescriptorTag
            ) | (
                DescriptorTag::TargetBackgroundGridDescriptorTag,
                DescriptorTag::TargetBackgroundGridDescriptorTag
            ) | (
                DescriptorTag::VideoWindowDescriptorTag,
                DescriptorTag::VideoWindowDescriptorTag
            ) | (
                DescriptorTag::CaDescriptorTag,
                DescriptorTag::CaDescriptorTag
            ) | (
                DescriptorTag::Iso639LanguageDescriptorTag,
                DescriptorTag::Iso639LanguageDescriptorTag
            ) | (
                DescriptorTag::SystemClockDescriptorTag,
                DescriptorTag::SystemClockDescriptorTag
            ) | (
                DescriptorTag::MultiplexBufferUtilizationDescriptorTag,
                DescriptorTag::MultiplexBufferUtilizationDescriptorTag
            ) | (
                DescriptorTag::CopyrightDescriptorTag,
                DescriptorTag::CopyrightDescriptorTag
            ) | (
                DescriptorTag::MaximumBitrateDescriptorTag,
                DescriptorTag::MaximumBitrateDescriptorTag
            ) | (
                DescriptorTag::PrivateDataIndicatorDescriptorTag,
                DescriptorTag::PrivateDataIndicatorDescriptorTag
            ) | (
                DescriptorTag::SmoothingBufferDescriptorTag,
                DescriptorTag::SmoothingBufferDescriptorTag
            ) | (
                DescriptorTag::StdDescriptorTag,
                DescriptorTag::StdDescriptorTag
            ) | (
                DescriptorTag::IbpDescriptorTag,
                DescriptorTag::IbpDescriptorTag
            ) | (
                DescriptorTag::Mpeg4VideoDescriptorTag,
                DescriptorTag::Mpeg4VideoDescriptorTag
            ) | (
                DescriptorTag::Mpeg4AudioDescriptorTag,
                DescriptorTag::Mpeg4AudioDescriptorTag
            ) | (
                DescriptorTag::IodDescriptorTag,
                DescriptorTag::IodDescriptorTag
            ) | (
                DescriptorTag::SlDescriptorTag,
                DescriptorTag::SlDescriptorTag
            ) | (
                DescriptorTag::FmcDescriptorTag,
                DescriptorTag::FmcDescriptorTag
            ) | (
                DescriptorTag::ExternalEsIdDescriptorTag,
                DescriptorTag::ExternalEsIdDescriptorTag
            ) | (
                DescriptorTag::MuxCodeDescriptorTag,
                DescriptorTag::MuxCodeDescriptorTag
            ) | (
                DescriptorTag::FmxBufferSizeDescriptorTag,
                DescriptorTag::FmxBufferSizeDescriptorTag
            ) | (
                DescriptorTag::MultiplexbufferDescriptorTag,
                DescriptorTag::MultiplexbufferDescriptorTag
            ) | (
                DescriptorTag::ContentLabelingDescriptorTag,
                DescriptorTag::ContentLabelingDescriptorTag
            ) | (
                DescriptorTag::MetadataPointerDescriptorTag,
                DescriptorTag::MetadataPointerDescriptorTag
            ) | (
                DescriptorTag::MetadataDescriptorTag,
                DescriptorTag::MetadataDescriptorTag
            ) | (
                DescriptorTag::MetadataStdDescriptorTag,
                DescriptorTag::MetadataStdDescriptorTag
            ) | (
                DescriptorTag::AvcVideoDescriptorTag,
                DescriptorTag::AvcVideoDescriptorTag
            ) | (
                DescriptorTag::AvcTimingAndHrdDescriptorTag,
                DescriptorTag::AvcTimingAndHrdDescriptorTag
            ) | (
                DescriptorTag::Mpeg2AacAudioDescriptorTag,
                DescriptorTag::Mpeg2AacAudioDescriptorTag
            ) | (
                DescriptorTag::FlexMuxTimingDescriptorTag,
                DescriptorTag::FlexMuxTimingDescriptorTag
            ) | (
                DescriptorTag::Mpeg4TextDescriptorTag,
                DescriptorTag::Mpeg4TextDescriptorTag
            ) | (
                DescriptorTag::Mpeg4AudioExtensionDescriptorTag,
                DescriptorTag::Mpeg4AudioExtensionDescriptorTag
            ) | (
                DescriptorTag::AuxiliaryVideoStreamDescriptorTag,
                DescriptorTag::AuxiliaryVideoStreamDescriptorTag
            ) | (
                DescriptorTag::SvcExtensionDescriptorTag,
                DescriptorTag::SvcExtensionDescriptorTag
            ) | (
                DescriptorTag::MvcExtensionDescriptorTag,
                DescriptorTag::MvcExtensionDescriptorTag
            ) | (
                DescriptorTag::J2kVideoDescriptorTag,
                DescriptorTag::J2kVideoDescriptorTag
            ) | (
                DescriptorTag::MvcOperationPointDescriptorTag,
                DescriptorTag::MvcOperationPointDescriptorTag
            ) | (
                DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptorTag,
                DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptorTag
            ) | (
                DescriptorTag::StereoscopicProgramInfoDescriptorTag,
                DescriptorTag::StereoscopicProgramInfoDescriptorTag
            ) | (
                DescriptorTag::StereoscopicVideoInfoDescriptorTag,
                DescriptorTag::StereoscopicVideoInfoDescriptorTag
            ) | (
                DescriptorTag::TransportProfileDescriptorTag,
                DescriptorTag::TransportProfileDescriptorTag
            ) | (
                DescriptorTag::HevcVideoDescriptorTag,
                DescriptorTag::HevcVideoDescriptorTag
            ) | (
                DescriptorTag::ExtensionDescriptorTag,
                DescriptorTag::ExtensionDescriptorTag
            ) | (DescriptorTag::UserPrivate, DescriptorTag::UserPrivate)
                | (DescriptorTag::Unknown, DescriptorTag::Unknown)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptor_type_equality() {
        assert_eq!(
            DescriptorTag::VideoStreamDescriptorTag,
            DescriptorTag::VideoStreamDescriptorTag
        );
        assert_eq!(
            DescriptorTag::AudioStreamDescriptorTag,
            DescriptorTag::AudioStreamDescriptorTag
        );
        assert_eq!(
            DescriptorTag::HierarchyDescriptorTag,
            DescriptorTag::HierarchyDescriptorTag
        );
        assert_eq!(
            DescriptorTag::RegistrationDescriptorTag,
            DescriptorTag::RegistrationDescriptorTag
        );
        assert_eq!(
            DescriptorTag::DataStreamAlignmentDescriptorTag,
            DescriptorTag::DataStreamAlignmentDescriptorTag
        );
        assert_eq!(
            DescriptorTag::TargetBackgroundGridDescriptorTag,
            DescriptorTag::TargetBackgroundGridDescriptorTag
        );
        assert_eq!(
            DescriptorTag::VideoWindowDescriptorTag,
            DescriptorTag::VideoWindowDescriptorTag
        );
        assert_eq!(
            DescriptorTag::CaDescriptorTag,
            DescriptorTag::CaDescriptorTag
        );
        assert_eq!(
            DescriptorTag::Iso639LanguageDescriptorTag,
            DescriptorTag::Iso639LanguageDescriptorTag
        );
        assert_eq!(
            DescriptorTag::SystemClockDescriptorTag,
            DescriptorTag::SystemClockDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MultiplexBufferUtilizationDescriptorTag,
            DescriptorTag::MultiplexBufferUtilizationDescriptorTag
        );
        assert_eq!(
            DescriptorTag::CopyrightDescriptorTag,
            DescriptorTag::CopyrightDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MaximumBitrateDescriptorTag,
            DescriptorTag::MaximumBitrateDescriptorTag
        );
        assert_eq!(
            DescriptorTag::PrivateDataIndicatorDescriptorTag,
            DescriptorTag::PrivateDataIndicatorDescriptorTag
        );
        assert_eq!(
            DescriptorTag::SmoothingBufferDescriptorTag,
            DescriptorTag::SmoothingBufferDescriptorTag
        );
        assert_eq!(
            DescriptorTag::StdDescriptorTag,
            DescriptorTag::StdDescriptorTag
        );
        assert_eq!(
            DescriptorTag::IbpDescriptorTag,
            DescriptorTag::IbpDescriptorTag
        );
        assert_eq!(
            DescriptorTag::Mpeg4VideoDescriptorTag,
            DescriptorTag::Mpeg4VideoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::Mpeg4AudioDescriptorTag,
            DescriptorTag::Mpeg4AudioDescriptorTag
        );
        assert_eq!(
            DescriptorTag::IodDescriptorTag,
            DescriptorTag::IodDescriptorTag
        );
        assert_eq!(
            DescriptorTag::SlDescriptorTag,
            DescriptorTag::SlDescriptorTag
        );
        assert_eq!(
            DescriptorTag::FmcDescriptorTag,
            DescriptorTag::FmcDescriptorTag
        );
        assert_eq!(
            DescriptorTag::ExternalEsIdDescriptorTag,
            DescriptorTag::ExternalEsIdDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MuxCodeDescriptorTag,
            DescriptorTag::MuxCodeDescriptorTag
        );
        assert_eq!(
            DescriptorTag::FmxBufferSizeDescriptorTag,
            DescriptorTag::FmxBufferSizeDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MultiplexbufferDescriptorTag,
            DescriptorTag::MultiplexbufferDescriptorTag
        );
        assert_eq!(
            DescriptorTag::ContentLabelingDescriptorTag,
            DescriptorTag::ContentLabelingDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MetadataPointerDescriptorTag,
            DescriptorTag::MetadataPointerDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MetadataDescriptorTag,
            DescriptorTag::MetadataDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MetadataStdDescriptorTag,
            DescriptorTag::MetadataStdDescriptorTag
        );
        assert_eq!(
            DescriptorTag::AvcVideoDescriptorTag,
            DescriptorTag::AvcVideoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::Mpeg2AacAudioDescriptorTag,
            DescriptorTag::Mpeg2AacAudioDescriptorTag
        );
        assert_eq!(
            DescriptorTag::FlexMuxTimingDescriptorTag,
            DescriptorTag::FlexMuxTimingDescriptorTag
        );
        assert_eq!(
            DescriptorTag::Mpeg4TextDescriptorTag,
            DescriptorTag::Mpeg4TextDescriptorTag
        );
        assert_eq!(
            DescriptorTag::Mpeg4AudioExtensionDescriptorTag,
            DescriptorTag::Mpeg4AudioExtensionDescriptorTag
        );
        assert_eq!(
            DescriptorTag::AuxiliaryVideoStreamDescriptorTag,
            DescriptorTag::AuxiliaryVideoStreamDescriptorTag
        );
        assert_eq!(
            DescriptorTag::SvcExtensionDescriptorTag,
            DescriptorTag::SvcExtensionDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MvcExtensionDescriptorTag,
            DescriptorTag::MvcExtensionDescriptorTag
        );
        assert_eq!(
            DescriptorTag::J2kVideoDescriptorTag,
            DescriptorTag::J2kVideoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::MvcOperationPointDescriptorTag,
            DescriptorTag::MvcOperationPointDescriptorTag
        );
        assert_eq!(
            DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptorTag,
            DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptorTag
        );
        assert_eq!(
            DescriptorTag::StereoscopicProgramInfoDescriptorTag,
            DescriptorTag::StereoscopicProgramInfoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::StereoscopicVideoInfoDescriptorTag,
            DescriptorTag::StereoscopicVideoInfoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::TransportProfileDescriptorTag,
            DescriptorTag::TransportProfileDescriptorTag
        );
        assert_eq!(
            DescriptorTag::HevcVideoDescriptorTag,
            DescriptorTag::HevcVideoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::ExtensionDescriptorTag,
            DescriptorTag::ExtensionDescriptorTag
        );
        assert_eq!(DescriptorTag::UserPrivate, DescriptorTag::UserPrivate);
        assert_eq!(DescriptorTag::Unknown, DescriptorTag::Unknown);
    }

    #[test]
    fn test_descriptor_type_inequality() {
        assert_ne!(
            DescriptorTag::VideoStreamDescriptorTag,
            DescriptorTag::AudioStreamDescriptorTag
        );
        assert_ne!(
            DescriptorTag::HierarchyDescriptorTag,
            DescriptorTag::RegistrationDescriptorTag
        );
        assert_ne!(
            DescriptorTag::DataStreamAlignmentDescriptorTag,
            DescriptorTag::TargetBackgroundGridDescriptorTag
        );
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(
            DescriptorTag::from(0x02),
            DescriptorTag::VideoStreamDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x03),
            DescriptorTag::AudioStreamDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x04),
            DescriptorTag::HierarchyDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x05),
            DescriptorTag::RegistrationDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x06),
            DescriptorTag::DataStreamAlignmentDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x07),
            DescriptorTag::TargetBackgroundGridDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x08),
            DescriptorTag::VideoWindowDescriptorTag
        );
        assert_eq!(DescriptorTag::from(0x09), DescriptorTag::CaDescriptorTag);
        assert_eq!(
            DescriptorTag::from(0x0A),
            DescriptorTag::Iso639LanguageDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x0B),
            DescriptorTag::SystemClockDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x0C),
            DescriptorTag::MultiplexBufferUtilizationDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x0D),
            DescriptorTag::CopyrightDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x0E),
            DescriptorTag::MaximumBitrateDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x0F),
            DescriptorTag::PrivateDataIndicatorDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x10),
            DescriptorTag::SmoothingBufferDescriptorTag
        );
        assert_eq!(DescriptorTag::from(0x11), DescriptorTag::StdDescriptorTag);
        assert_eq!(DescriptorTag::from(0x12), DescriptorTag::IbpDescriptorTag);
        assert_eq!(
            DescriptorTag::from(0x1B),
            DescriptorTag::Mpeg4VideoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x1C),
            DescriptorTag::Mpeg4AudioDescriptorTag
        );
        assert_eq!(DescriptorTag::from(0x1D), DescriptorTag::IodDescriptorTag);
        assert_eq!(DescriptorTag::from(0x1E), DescriptorTag::SlDescriptorTag);
        assert_eq!(DescriptorTag::from(0x1F), DescriptorTag::FmcDescriptorTag);
        assert_eq!(
            DescriptorTag::from(0x20),
            DescriptorTag::ExternalEsIdDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x21),
            DescriptorTag::MuxCodeDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x22),
            DescriptorTag::FmxBufferSizeDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x23),
            DescriptorTag::MultiplexbufferDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x24),
            DescriptorTag::ContentLabelingDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x25),
            DescriptorTag::MetadataPointerDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x26),
            DescriptorTag::MetadataDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x27),
            DescriptorTag::MetadataStdDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x28),
            DescriptorTag::AvcVideoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x2A),
            DescriptorTag::AvcTimingAndHrdDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x2B),
            DescriptorTag::Mpeg2AacAudioDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x2C),
            DescriptorTag::FlexMuxTimingDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x2D),
            DescriptorTag::Mpeg4TextDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x2E),
            DescriptorTag::Mpeg4AudioExtensionDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x2F),
            DescriptorTag::AuxiliaryVideoStreamDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x30),
            DescriptorTag::SvcExtensionDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x31),
            DescriptorTag::MvcExtensionDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x32),
            DescriptorTag::J2kVideoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x33),
            DescriptorTag::MvcOperationPointDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x34),
            DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x35),
            DescriptorTag::StereoscopicProgramInfoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x36),
            DescriptorTag::StereoscopicVideoInfoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x37),
            DescriptorTag::TransportProfileDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x38),
            DescriptorTag::HevcVideoDescriptorTag
        );
        assert_eq!(
            DescriptorTag::from(0x3F),
            DescriptorTag::ExtensionDescriptorTag
        );
        assert_eq!(DescriptorTag::from(0x40), DescriptorTag::UserPrivate);
        assert_eq!(DescriptorTag::from(0xFF), DescriptorTag::UserPrivate);
        assert_eq!(DescriptorTag::from(0x00), DescriptorTag::Unknown);
    }
}
