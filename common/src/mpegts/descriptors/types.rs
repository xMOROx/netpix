use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub enum DescriptorTag {
    VideoStreamDescriptor,
    AudioStreamDescriptor,
    HierarchyDescriptor,
    RegistrationDescriptor,
    DataStreamAlignmentDescriptor,
    TargetBackgroundGridDescriptor,
    VideoWindowDescriptor,
    CaDescriptor,
    Iso639LanguageDescriptor,
    SystemClockDescriptor,
    MultiplexBufferUtilizationDescriptor,
    CopyrightDescriptor,
    MaximumBitrateDescriptor,
    PrivateDataIndicatorDescriptor,
    SmoothingBufferDescriptor,
    StdDescriptor,
    IbpDescriptor,
    Mpeg4VideoDescriptor,
    Mpeg4AudioDescriptor,
    IodDescriptor,
    SlDescriptor,
    FmcDescriptor,
    ExternalEsIdDescriptor,
    MuxCodeDescriptor,
    FmxBufferSizeDescriptor,
    MultiplexbufferDescriptor,
    ContentLabelingDescriptor,
    MetadataPointerDescriptor,
    MetadataDescriptor,
    MetadataStdDescriptor,
    AvcVideoDescriptor,
    AvcTimingAndHrdDescriptor,
    Mpeg2AacAudioDescriptor,
    FlexMuxTimingDescriptor,
    Mpeg4TextDescriptor,
    Mpeg4AudioExtensionDescriptor,
    AuxiliaryVideoStreamDescriptor,
    SvcExtensionDescriptor,
    MvcExtensionDescriptor,
    J2kVideoDescriptor,
    MvcOperationPointDescriptor,
    Mpeg2StereoscopicVideoFormatDescriptor,
    StereoscopicProgramInfoDescriptor,
    StereoscopicVideoInfoDescriptor,
    TransportProfileDescriptor,
    HevcVideoDescriptor,
    ExtensionDescriptor,
    UserPrivate,
    Unknown,
}

impl DescriptorTag {
    pub fn to_u8(&self) -> u8 {
        match self {
            DescriptorTag::VideoStreamDescriptor => 0x02,
            DescriptorTag::AudioStreamDescriptor => 0x03,
            DescriptorTag::HierarchyDescriptor => 0x04,
            DescriptorTag::RegistrationDescriptor => 0x05,
            DescriptorTag::DataStreamAlignmentDescriptor => 0x06,
            DescriptorTag::TargetBackgroundGridDescriptor => 0x07,
            DescriptorTag::VideoWindowDescriptor => 0x08,
            DescriptorTag::CaDescriptor => 0x09,
            DescriptorTag::Iso639LanguageDescriptor => 0x0A,
            DescriptorTag::SystemClockDescriptor => 0x0B,
            DescriptorTag::MultiplexBufferUtilizationDescriptor => 0x0C,
            DescriptorTag::CopyrightDescriptor => 0x0D,
            DescriptorTag::MaximumBitrateDescriptor => 0x0E,
            DescriptorTag::PrivateDataIndicatorDescriptor => 0x0F,
            DescriptorTag::SmoothingBufferDescriptor => 0x10,
            DescriptorTag::StdDescriptor => 0x11,
            DescriptorTag::IbpDescriptor => 0x12,
            DescriptorTag::Mpeg4VideoDescriptor => 0x1B,
            DescriptorTag::Mpeg4AudioDescriptor => 0x1C,
            DescriptorTag::IodDescriptor => 0x1D,
            DescriptorTag::SlDescriptor => 0x1E,
            DescriptorTag::FmcDescriptor => 0x1F,
            DescriptorTag::ExternalEsIdDescriptor => 0x20,
            DescriptorTag::MuxCodeDescriptor => 0x21,
            DescriptorTag::FmxBufferSizeDescriptor => 0x22,
            DescriptorTag::MultiplexbufferDescriptor => 0x23,
            DescriptorTag::ContentLabelingDescriptor => 0x24,
            DescriptorTag::MetadataPointerDescriptor => 0x25,
            DescriptorTag::MetadataDescriptor => 0x26,
            DescriptorTag::MetadataStdDescriptor => 0x27,
            DescriptorTag::AvcVideoDescriptor => 0x28,
            DescriptorTag::AvcTimingAndHrdDescriptor => 0x2A,
            DescriptorTag::Mpeg2AacAudioDescriptor => 0x2B,
            DescriptorTag::FlexMuxTimingDescriptor => 0x2C,
            DescriptorTag::Mpeg4TextDescriptor => 0x2D,
            DescriptorTag::Mpeg4AudioExtensionDescriptor => 0x2E,
            DescriptorTag::AuxiliaryVideoStreamDescriptor => 0x2F,
            DescriptorTag::SvcExtensionDescriptor => 0x30,
            DescriptorTag::MvcExtensionDescriptor => 0x31,
            DescriptorTag::J2kVideoDescriptor => 0x32,
            DescriptorTag::MvcOperationPointDescriptor => 0x33,
            DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptor => 0x34,
            DescriptorTag::StereoscopicProgramInfoDescriptor => 0x35,
            DescriptorTag::StereoscopicVideoInfoDescriptor => 0x36,
            DescriptorTag::TransportProfileDescriptor => 0x37,
            DescriptorTag::HevcVideoDescriptor => 0x38,
            DescriptorTag::ExtensionDescriptor => 0x3F,
            DescriptorTag::UserPrivate => 0x40,
            DescriptorTag::Unknown => 0x00,
        }
    }
}


impl Default for DescriptorTag {
    fn default() -> Self {
        DescriptorTag::Unknown
    }
}

impl From<u8> for DescriptorTag {
    fn from(value: u8) -> Self {
        match value {
            0x02 => DescriptorTag::VideoStreamDescriptor,
            0x03 => DescriptorTag::AudioStreamDescriptor,
            0x04 => DescriptorTag::HierarchyDescriptor,
            0x05 => DescriptorTag::RegistrationDescriptor,
            0x06 => DescriptorTag::DataStreamAlignmentDescriptor,
            0x07 => DescriptorTag::TargetBackgroundGridDescriptor,
            0x08 => DescriptorTag::VideoWindowDescriptor,
            0x09 => DescriptorTag::CaDescriptor,
            0x0A => DescriptorTag::Iso639LanguageDescriptor,
            0x0B => DescriptorTag::SystemClockDescriptor,
            0x0C => DescriptorTag::MultiplexBufferUtilizationDescriptor,
            0x0D => DescriptorTag::CopyrightDescriptor,
            0x0E => DescriptorTag::MaximumBitrateDescriptor,
            0x0F => DescriptorTag::PrivateDataIndicatorDescriptor,
            0x10 => DescriptorTag::SmoothingBufferDescriptor,
            0x11 => DescriptorTag::StdDescriptor,
            0x12 => DescriptorTag::IbpDescriptor,
            0x1B => DescriptorTag::Mpeg4VideoDescriptor,
            0x1C => DescriptorTag::Mpeg4AudioDescriptor,
            0x1D => DescriptorTag::IodDescriptor,
            0x1E => DescriptorTag::SlDescriptor,
            0x1F => DescriptorTag::FmcDescriptor,
            0x20 => DescriptorTag::ExternalEsIdDescriptor,
            0x21 => DescriptorTag::MuxCodeDescriptor,
            0x22 => DescriptorTag::FmxBufferSizeDescriptor,
            0x23 => DescriptorTag::MultiplexbufferDescriptor,
            0x24 => DescriptorTag::ContentLabelingDescriptor,
            0x25 => DescriptorTag::MetadataPointerDescriptor,
            0x26 => DescriptorTag::MetadataDescriptor,
            0x27 => DescriptorTag::MetadataStdDescriptor,
            0x28 => DescriptorTag::AvcVideoDescriptor,
            0x2A => DescriptorTag::AvcTimingAndHrdDescriptor,
            0x2B => DescriptorTag::Mpeg2AacAudioDescriptor,
            0x2C => DescriptorTag::FlexMuxTimingDescriptor,
            0x2D => DescriptorTag::Mpeg4TextDescriptor,
            0x2E => DescriptorTag::Mpeg4AudioExtensionDescriptor,
            0x2F => DescriptorTag::AuxiliaryVideoStreamDescriptor,
            0x30 => DescriptorTag::SvcExtensionDescriptor,
            0x31 => DescriptorTag::MvcExtensionDescriptor,
            0x32 => DescriptorTag::J2kVideoDescriptor,
            0x33 => DescriptorTag::MvcOperationPointDescriptor,
            0x34 => DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptor,
            0x35 => DescriptorTag::StereoscopicProgramInfoDescriptor,
            0x36 => DescriptorTag::StereoscopicVideoInfoDescriptor,
            0x37 => DescriptorTag::TransportProfileDescriptor,
            0x38 => DescriptorTag::HevcVideoDescriptor,
            0x3F => DescriptorTag::ExtensionDescriptor,
            0x40..=0xFF => DescriptorTag::UserPrivate,
            _ => DescriptorTag::Unknown,
        }
    }
}

impl PartialEq for DescriptorTag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DescriptorTag::VideoStreamDescriptor, DescriptorTag::VideoStreamDescriptor) => true,
            (DescriptorTag::AudioStreamDescriptor, DescriptorTag::AudioStreamDescriptor) => true,
            (DescriptorTag::HierarchyDescriptor, DescriptorTag::HierarchyDescriptor) => true,
            (DescriptorTag::RegistrationDescriptor, DescriptorTag::RegistrationDescriptor) => true,
            (DescriptorTag::DataStreamAlignmentDescriptor, DescriptorTag::DataStreamAlignmentDescriptor) => true,
            (DescriptorTag::TargetBackgroundGridDescriptor, DescriptorTag::TargetBackgroundGridDescriptor) => true,
            (DescriptorTag::VideoWindowDescriptor, DescriptorTag::VideoWindowDescriptor) => true,
            (DescriptorTag::CaDescriptor, DescriptorTag::CaDescriptor) => true,
            (DescriptorTag::Iso639LanguageDescriptor, DescriptorTag::Iso639LanguageDescriptor) => true,
            (DescriptorTag::SystemClockDescriptor, DescriptorTag::SystemClockDescriptor) => true,
            (DescriptorTag::MultiplexBufferUtilizationDescriptor, DescriptorTag::MultiplexBufferUtilizationDescriptor) => true,
            (DescriptorTag::CopyrightDescriptor, DescriptorTag::CopyrightDescriptor) => true,
            (DescriptorTag::MaximumBitrateDescriptor, DescriptorTag::MaximumBitrateDescriptor) => true,
            (DescriptorTag::PrivateDataIndicatorDescriptor, DescriptorTag::PrivateDataIndicatorDescriptor) => true,
            (DescriptorTag::SmoothingBufferDescriptor, DescriptorTag::SmoothingBufferDescriptor) => true,
            (DescriptorTag::StdDescriptor, DescriptorTag::StdDescriptor) => true,
            (DescriptorTag::IbpDescriptor, DescriptorTag::IbpDescriptor) => true,
            (DescriptorTag::Mpeg4VideoDescriptor, DescriptorTag::Mpeg4VideoDescriptor) => true,
            (DescriptorTag::Mpeg4AudioDescriptor, DescriptorTag::Mpeg4AudioDescriptor) => true,
            (DescriptorTag::IodDescriptor, DescriptorTag::IodDescriptor) => true,
            (DescriptorTag::SlDescriptor, DescriptorTag::SlDescriptor) => true,
            (DescriptorTag::FmcDescriptor, DescriptorTag::FmcDescriptor) => true,
            (DescriptorTag::ExternalEsIdDescriptor, DescriptorTag::ExternalEsIdDescriptor) => true,
            (DescriptorTag::MuxCodeDescriptor, DescriptorTag::MuxCodeDescriptor) => true,
            (DescriptorTag::FmxBufferSizeDescriptor, DescriptorTag::FmxBufferSizeDescriptor) => true,
            (DescriptorTag::MultiplexbufferDescriptor, DescriptorTag::MultiplexbufferDescriptor) => true,
            (DescriptorTag::ContentLabelingDescriptor, DescriptorTag::ContentLabelingDescriptor) => true,
            (DescriptorTag::MetadataPointerDescriptor, DescriptorTag::MetadataPointerDescriptor) => true,
            (DescriptorTag::MetadataDescriptor, DescriptorTag::MetadataDescriptor) => true,
            (DescriptorTag::MetadataStdDescriptor, DescriptorTag::MetadataStdDescriptor) => true,
            (DescriptorTag::AvcVideoDescriptor, DescriptorTag::AvcVideoDescriptor) => true,
            (DescriptorTag::AvcTimingAndHrdDescriptor, DescriptorTag::AvcTimingAndHrdDescriptor) => true,
            (DescriptorTag::Mpeg2AacAudioDescriptor, DescriptorTag::Mpeg2AacAudioDescriptor) => true,
            (DescriptorTag::FlexMuxTimingDescriptor, DescriptorTag::FlexMuxTimingDescriptor) => true,
            (DescriptorTag::Mpeg4TextDescriptor, DescriptorTag::Mpeg4TextDescriptor) => true,
            (DescriptorTag::Mpeg4AudioExtensionDescriptor, DescriptorTag::Mpeg4AudioExtensionDescriptor) => true,
            (DescriptorTag::AuxiliaryVideoStreamDescriptor, DescriptorTag::AuxiliaryVideoStreamDescriptor) => true,
            (DescriptorTag::SvcExtensionDescriptor, DescriptorTag::SvcExtensionDescriptor) => true,
            (DescriptorTag::MvcExtensionDescriptor, DescriptorTag::MvcExtensionDescriptor) => true,
            (DescriptorTag::J2kVideoDescriptor, DescriptorTag::J2kVideoDescriptor) => true,
            (DescriptorTag::MvcOperationPointDescriptor, DescriptorTag::MvcOperationPointDescriptor) => true,
            (DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptor, DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptor) => true,
            (DescriptorTag::StereoscopicProgramInfoDescriptor, DescriptorTag::StereoscopicProgramInfoDescriptor) => true,
            (DescriptorTag::StereoscopicVideoInfoDescriptor, DescriptorTag::StereoscopicVideoInfoDescriptor) => true,
            (DescriptorTag::TransportProfileDescriptor, DescriptorTag::TransportProfileDescriptor) => true,
            (DescriptorTag::HevcVideoDescriptor, DescriptorTag::HevcVideoDescriptor) => true,
            (DescriptorTag::ExtensionDescriptor, DescriptorTag::ExtensionDescriptor) => true,
            (DescriptorTag::UserPrivate, DescriptorTag::UserPrivate) => true,
            (DescriptorTag::Unknown, DescriptorTag::Unknown) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptor_type_equality() {
        assert_eq!(DescriptorTag::VideoStreamDescriptor, DescriptorTag::VideoStreamDescriptor);
        assert_eq!(DescriptorTag::AudioStreamDescriptor, DescriptorTag::AudioStreamDescriptor);
        assert_eq!(DescriptorTag::HierarchyDescriptor, DescriptorTag::HierarchyDescriptor);
        assert_eq!(DescriptorTag::RegistrationDescriptor, DescriptorTag::RegistrationDescriptor);
        assert_eq!(DescriptorTag::DataStreamAlignmentDescriptor, DescriptorTag::DataStreamAlignmentDescriptor);
        assert_eq!(DescriptorTag::TargetBackgroundGridDescriptor, DescriptorTag::TargetBackgroundGridDescriptor);
        assert_eq!(DescriptorTag::VideoWindowDescriptor, DescriptorTag::VideoWindowDescriptor);
        assert_eq!(DescriptorTag::CaDescriptor, DescriptorTag::CaDescriptor);
        assert_eq!(DescriptorTag::Iso639LanguageDescriptor, DescriptorTag::Iso639LanguageDescriptor);
        assert_eq!(DescriptorTag::SystemClockDescriptor, DescriptorTag::SystemClockDescriptor);
        assert_eq!(DescriptorTag::MultiplexBufferUtilizationDescriptor, DescriptorTag::MultiplexBufferUtilizationDescriptor);
        assert_eq!(DescriptorTag::CopyrightDescriptor, DescriptorTag::CopyrightDescriptor);
        assert_eq!(DescriptorTag::MaximumBitrateDescriptor, DescriptorTag::MaximumBitrateDescriptor);
        assert_eq!(DescriptorTag::PrivateDataIndicatorDescriptor, DescriptorTag::PrivateDataIndicatorDescriptor);
        assert_eq!(DescriptorTag::SmoothingBufferDescriptor, DescriptorTag::SmoothingBufferDescriptor);
        assert_eq!(DescriptorTag::StdDescriptor, DescriptorTag::StdDescriptor);
        assert_eq!(DescriptorTag::IbpDescriptor, DescriptorTag::IbpDescriptor);
        assert_eq!(DescriptorTag::Mpeg4VideoDescriptor, DescriptorTag::Mpeg4VideoDescriptor);
        assert_eq!(DescriptorTag::Mpeg4AudioDescriptor, DescriptorTag::Mpeg4AudioDescriptor);
        assert_eq!(DescriptorTag::IodDescriptor, DescriptorTag::IodDescriptor);
        assert_eq!(DescriptorTag::SlDescriptor, DescriptorTag::SlDescriptor);
        assert_eq!(DescriptorTag::FmcDescriptor, DescriptorTag::FmcDescriptor);
        assert_eq!(DescriptorTag::ExternalEsIdDescriptor, DescriptorTag::ExternalEsIdDescriptor);
        assert_eq!(DescriptorTag::MuxCodeDescriptor, DescriptorTag::MuxCodeDescriptor);
        assert_eq!(DescriptorTag::FmxBufferSizeDescriptor, DescriptorTag::FmxBufferSizeDescriptor);
        assert_eq!(DescriptorTag::MultiplexbufferDescriptor, DescriptorTag::MultiplexbufferDescriptor);
        assert_eq!(DescriptorTag::ContentLabelingDescriptor, DescriptorTag::ContentLabelingDescriptor);
        assert_eq!(DescriptorTag::MetadataPointerDescriptor, DescriptorTag::MetadataPointerDescriptor);
        assert_eq!(DescriptorTag::MetadataDescriptor, DescriptorTag::MetadataDescriptor);
        assert_eq!(DescriptorTag::MetadataStdDescriptor, DescriptorTag::MetadataStdDescriptor);
        assert_eq!(DescriptorTag::AvcVideoDescriptor, DescriptorTag::AvcVideoDescriptor);
        assert_eq!(DescriptorTag::Mpeg2AacAudioDescriptor, DescriptorTag::Mpeg2AacAudioDescriptor);
        assert_eq!(DescriptorTag::FlexMuxTimingDescriptor, DescriptorTag::FlexMuxTimingDescriptor);
        assert_eq!(DescriptorTag::Mpeg4TextDescriptor, DescriptorTag::Mpeg4TextDescriptor);
        assert_eq!(DescriptorTag::Mpeg4AudioExtensionDescriptor, DescriptorTag::Mpeg4AudioExtensionDescriptor);
        assert_eq!(DescriptorTag::AuxiliaryVideoStreamDescriptor, DescriptorTag::AuxiliaryVideoStreamDescriptor);
        assert_eq!(DescriptorTag::SvcExtensionDescriptor, DescriptorTag::SvcExtensionDescriptor);
        assert_eq!(DescriptorTag::MvcExtensionDescriptor, DescriptorTag::MvcExtensionDescriptor);
        assert_eq!(DescriptorTag::J2kVideoDescriptor, DescriptorTag::J2kVideoDescriptor);
        assert_eq!(DescriptorTag::MvcOperationPointDescriptor, DescriptorTag::MvcOperationPointDescriptor);
        assert_eq!(DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptor, DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptor);
        assert_eq!(DescriptorTag::StereoscopicProgramInfoDescriptor, DescriptorTag::StereoscopicProgramInfoDescriptor);
        assert_eq!(DescriptorTag::StereoscopicVideoInfoDescriptor, DescriptorTag::StereoscopicVideoInfoDescriptor);
        assert_eq!(DescriptorTag::TransportProfileDescriptor, DescriptorTag::TransportProfileDescriptor);
        assert_eq!(DescriptorTag::HevcVideoDescriptor, DescriptorTag::HevcVideoDescriptor);
        assert_eq!(DescriptorTag::ExtensionDescriptor, DescriptorTag::ExtensionDescriptor);
        assert_eq!(DescriptorTag::UserPrivate, DescriptorTag::UserPrivate);
        assert_eq!(DescriptorTag::Unknown, DescriptorTag::Unknown);
    }

    #[test]
    fn test_descriptor_type_inequality() {
        assert_ne!(DescriptorTag::VideoStreamDescriptor, DescriptorTag::AudioStreamDescriptor);
        assert_ne!(DescriptorTag::HierarchyDescriptor, DescriptorTag::RegistrationDescriptor);
        assert_ne!(DescriptorTag::DataStreamAlignmentDescriptor, DescriptorTag::TargetBackgroundGridDescriptor);
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(DescriptorTag::from(0x02), DescriptorTag::VideoStreamDescriptor);
        assert_eq!(DescriptorTag::from(0x03), DescriptorTag::AudioStreamDescriptor);
        assert_eq!(DescriptorTag::from(0x04), DescriptorTag::HierarchyDescriptor);
        assert_eq!(DescriptorTag::from(0x05), DescriptorTag::RegistrationDescriptor);
        assert_eq!(DescriptorTag::from(0x06), DescriptorTag::DataStreamAlignmentDescriptor);
        assert_eq!(DescriptorTag::from(0x07), DescriptorTag::TargetBackgroundGridDescriptor);
        assert_eq!(DescriptorTag::from(0x08), DescriptorTag::VideoWindowDescriptor);
        assert_eq!(DescriptorTag::from(0x09), DescriptorTag::CaDescriptor);
        assert_eq!(DescriptorTag::from(0x0A), DescriptorTag::Iso639LanguageDescriptor);
        assert_eq!(DescriptorTag::from(0x0B), DescriptorTag::SystemClockDescriptor);
        assert_eq!(DescriptorTag::from(0x0C), DescriptorTag::MultiplexBufferUtilizationDescriptor);
        assert_eq!(DescriptorTag::from(0x0D), DescriptorTag::CopyrightDescriptor);
        assert_eq!(DescriptorTag::from(0x0E), DescriptorTag::MaximumBitrateDescriptor);
        assert_eq!(DescriptorTag::from(0x0F), DescriptorTag::PrivateDataIndicatorDescriptor);
        assert_eq!(DescriptorTag::from(0x10), DescriptorTag::SmoothingBufferDescriptor);
        assert_eq!(DescriptorTag::from(0x11), DescriptorTag::StdDescriptor);
        assert_eq!(DescriptorTag::from(0x12), DescriptorTag::IbpDescriptor);
        assert_eq!(DescriptorTag::from(0x1B), DescriptorTag::Mpeg4VideoDescriptor);
        assert_eq!(DescriptorTag::from(0x1C), DescriptorTag::Mpeg4AudioDescriptor);
        assert_eq!(DescriptorTag::from(0x1D), DescriptorTag::IodDescriptor);
        assert_eq!(DescriptorTag::from(0x1E), DescriptorTag::SlDescriptor);
        assert_eq!(DescriptorTag::from(0x1F), DescriptorTag::FmcDescriptor);
        assert_eq!(DescriptorTag::from(0x20), DescriptorTag::ExternalEsIdDescriptor);
        assert_eq!(DescriptorTag::from(0x21), DescriptorTag::MuxCodeDescriptor);
        assert_eq!(DescriptorTag::from(0x22), DescriptorTag::FmxBufferSizeDescriptor);
        assert_eq!(DescriptorTag::from(0x23), DescriptorTag::MultiplexbufferDescriptor);
        assert_eq!(DescriptorTag::from(0x24), DescriptorTag::ContentLabelingDescriptor);
        assert_eq!(DescriptorTag::from(0x25), DescriptorTag::MetadataPointerDescriptor);
        assert_eq!(DescriptorTag::from(0x26), DescriptorTag::MetadataDescriptor);
        assert_eq!(DescriptorTag::from(0x27), DescriptorTag::MetadataStdDescriptor);
        assert_eq!(DescriptorTag::from(0x28), DescriptorTag::AvcVideoDescriptor);
        assert_eq!(DescriptorTag::from(0x2A), DescriptorTag::AvcTimingAndHrdDescriptor);
        assert_eq!(DescriptorTag::from(0x2B), DescriptorTag::Mpeg2AacAudioDescriptor);
        assert_eq!(DescriptorTag::from(0x2C), DescriptorTag::FlexMuxTimingDescriptor);
        assert_eq!(DescriptorTag::from(0x2D), DescriptorTag::Mpeg4TextDescriptor);
        assert_eq!(DescriptorTag::from(0x2E), DescriptorTag::Mpeg4AudioExtensionDescriptor);
        assert_eq!(DescriptorTag::from(0x2F), DescriptorTag::AuxiliaryVideoStreamDescriptor);
        assert_eq!(DescriptorTag::from(0x30), DescriptorTag::SvcExtensionDescriptor);
        assert_eq!(DescriptorTag::from(0x31), DescriptorTag::MvcExtensionDescriptor);
        assert_eq!(DescriptorTag::from(0x32), DescriptorTag::J2kVideoDescriptor);
        assert_eq!(DescriptorTag::from(0x33), DescriptorTag::MvcOperationPointDescriptor);
        assert_eq!(DescriptorTag::from(0x34), DescriptorTag::Mpeg2StereoscopicVideoFormatDescriptor);
        assert_eq!(DescriptorTag::from(0x35), DescriptorTag::StereoscopicProgramInfoDescriptor);
        assert_eq!(DescriptorTag::from(0x36), DescriptorTag::StereoscopicVideoInfoDescriptor);
        assert_eq!(DescriptorTag::from(0x37), DescriptorTag::TransportProfileDescriptor);
        assert_eq!(DescriptorTag::from(0x38), DescriptorTag::HevcVideoDescriptor);
        assert_eq!(DescriptorTag::from(0x3F), DescriptorTag::ExtensionDescriptor);
        assert_eq!(DescriptorTag::from(0x40), DescriptorTag::UserPrivate);
        assert_eq!(DescriptorTag::from(0xFF), DescriptorTag::UserPrivate);
        assert_eq!(DescriptorTag::from(0x00), DescriptorTag::Unknown);
    }
}