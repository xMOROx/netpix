use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub enum DescriptorType {
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

impl DescriptorType {
    pub fn to_u8(&self) -> u8 {
        match self {
            DescriptorType::VideoStreamDescriptor => 0x02,
            DescriptorType::AudioStreamDescriptor => 0x03,
            DescriptorType::HierarchyDescriptor => 0x04,
            DescriptorType::RegistrationDescriptor => 0x05,
            DescriptorType::DataStreamAlignmentDescriptor => 0x06,
            DescriptorType::TargetBackgroundGridDescriptor => 0x07,
            DescriptorType::VideoWindowDescriptor => 0x08,
            DescriptorType::CaDescriptor => 0x09,
            DescriptorType::Iso639LanguageDescriptor => 0x0A,
            DescriptorType::SystemClockDescriptor => 0x0B,
            DescriptorType::MultiplexBufferUtilizationDescriptor => 0x0C,
            DescriptorType::CopyrightDescriptor => 0x0D,
            DescriptorType::MaximumBitrateDescriptor => 0x0E,
            DescriptorType::PrivateDataIndicatorDescriptor => 0x0F,
            DescriptorType::SmoothingBufferDescriptor => 0x10,
            DescriptorType::StdDescriptor => 0x11,
            DescriptorType::IbpDescriptor => 0x12,
            DescriptorType::Mpeg4VideoDescriptor => 0x1B,
            DescriptorType::Mpeg4AudioDescriptor => 0x1C,
            DescriptorType::IodDescriptor => 0x1D,
            DescriptorType::SlDescriptor => 0x1E,
            DescriptorType::FmcDescriptor => 0x1F,
            DescriptorType::ExternalEsIdDescriptor => 0x20,
            DescriptorType::MuxCodeDescriptor => 0x21,
            DescriptorType::FmxBufferSizeDescriptor => 0x22,
            DescriptorType::MultiplexbufferDescriptor => 0x23,
            DescriptorType::ContentLabelingDescriptor => 0x24,
            DescriptorType::MetadataPointerDescriptor => 0x25,
            DescriptorType::MetadataDescriptor => 0x26,
            DescriptorType::MetadataStdDescriptor => 0x27,
            DescriptorType::AvcVideoDescriptor => 0x28,
            DescriptorType::AvcTimingAndHrdDescriptor => 0x2A,
            DescriptorType::Mpeg2AacAudioDescriptor => 0x2B,
            DescriptorType::FlexMuxTimingDescriptor => 0x2C,
            DescriptorType::Mpeg4TextDescriptor => 0x2D,
            DescriptorType::Mpeg4AudioExtensionDescriptor => 0x2E,
            DescriptorType::AuxiliaryVideoStreamDescriptor => 0x2F,
            DescriptorType::SvcExtensionDescriptor => 0x30,
            DescriptorType::MvcExtensionDescriptor => 0x31,
            DescriptorType::J2kVideoDescriptor => 0x32,
            DescriptorType::MvcOperationPointDescriptor => 0x33,
            DescriptorType::Mpeg2StereoscopicVideoFormatDescriptor => 0x34,
            DescriptorType::StereoscopicProgramInfoDescriptor => 0x35,
            DescriptorType::StereoscopicVideoInfoDescriptor => 0x36,
            DescriptorType::TransportProfileDescriptor => 0x37,
            DescriptorType::HevcVideoDescriptor => 0x38,
            DescriptorType::ExtensionDescriptor => 0x3F,
            DescriptorType::UserPrivate => 0x40,
            DescriptorType::Unknown => 0x00,
        }
    }
}


impl Default for DescriptorType {
    fn default() -> Self {
        DescriptorType::Unknown
    }
}

impl From<u8> for DescriptorType {
    fn from(value: u8) -> Self {
        match value {
            0x02 => DescriptorType::VideoStreamDescriptor,
            0x03 => DescriptorType::AudioStreamDescriptor,
            0x04 => DescriptorType::HierarchyDescriptor,
            0x05 => DescriptorType::RegistrationDescriptor,
            0x06 => DescriptorType::DataStreamAlignmentDescriptor,
            0x07 => DescriptorType::TargetBackgroundGridDescriptor,
            0x08 => DescriptorType::VideoWindowDescriptor,
            0x09 => DescriptorType::CaDescriptor,
            0x0A => DescriptorType::Iso639LanguageDescriptor,
            0x0B => DescriptorType::SystemClockDescriptor,
            0x0C => DescriptorType::MultiplexBufferUtilizationDescriptor,
            0x0D => DescriptorType::CopyrightDescriptor,
            0x0E => DescriptorType::MaximumBitrateDescriptor,
            0x0F => DescriptorType::PrivateDataIndicatorDescriptor,
            0x10 => DescriptorType::SmoothingBufferDescriptor,
            0x11 => DescriptorType::StdDescriptor,
            0x12 => DescriptorType::IbpDescriptor,
            0x1B => DescriptorType::Mpeg4VideoDescriptor,
            0x1C => DescriptorType::Mpeg4AudioDescriptor,
            0x1D => DescriptorType::IodDescriptor,
            0x1E => DescriptorType::SlDescriptor,
            0x1F => DescriptorType::FmcDescriptor,
            0x20 => DescriptorType::ExternalEsIdDescriptor,
            0x21 => DescriptorType::MuxCodeDescriptor,
            0x22 => DescriptorType::FmxBufferSizeDescriptor,
            0x23 => DescriptorType::MultiplexbufferDescriptor,
            0x24 => DescriptorType::ContentLabelingDescriptor,
            0x25 => DescriptorType::MetadataPointerDescriptor,
            0x26 => DescriptorType::MetadataDescriptor,
            0x27 => DescriptorType::MetadataStdDescriptor,
            0x28 => DescriptorType::AvcVideoDescriptor,
            0x2A => DescriptorType::AvcTimingAndHrdDescriptor,
            0x2B => DescriptorType::Mpeg2AacAudioDescriptor,
            0x2C => DescriptorType::FlexMuxTimingDescriptor,
            0x2D => DescriptorType::Mpeg4TextDescriptor,
            0x2E => DescriptorType::Mpeg4AudioExtensionDescriptor,
            0x2F => DescriptorType::AuxiliaryVideoStreamDescriptor,
            0x30 => DescriptorType::SvcExtensionDescriptor,
            0x31 => DescriptorType::MvcExtensionDescriptor,
            0x32 => DescriptorType::J2kVideoDescriptor,
            0x33 => DescriptorType::MvcOperationPointDescriptor,
            0x34 => DescriptorType::Mpeg2StereoscopicVideoFormatDescriptor,
            0x35 => DescriptorType::StereoscopicProgramInfoDescriptor,
            0x36 => DescriptorType::StereoscopicVideoInfoDescriptor,
            0x37 => DescriptorType::TransportProfileDescriptor,
            0x38 => DescriptorType::HevcVideoDescriptor,
            0x3F => DescriptorType::ExtensionDescriptor,
            0x40..=0xFF => DescriptorType::UserPrivate,
            _ => DescriptorType::Unknown,
        }
    }
}

impl PartialEq for DescriptorType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DescriptorType::VideoStreamDescriptor, DescriptorType::VideoStreamDescriptor) => true,
            (DescriptorType::AudioStreamDescriptor, DescriptorType::AudioStreamDescriptor) => true,
            (DescriptorType::HierarchyDescriptor, DescriptorType::HierarchyDescriptor) => true,
            (DescriptorType::RegistrationDescriptor, DescriptorType::RegistrationDescriptor) => true,
            (DescriptorType::DataStreamAlignmentDescriptor, DescriptorType::DataStreamAlignmentDescriptor) => true,
            (DescriptorType::TargetBackgroundGridDescriptor, DescriptorType::TargetBackgroundGridDescriptor) => true,
            (DescriptorType::VideoWindowDescriptor, DescriptorType::VideoWindowDescriptor) => true,
            (DescriptorType::CaDescriptor, DescriptorType::CaDescriptor) => true,
            (DescriptorType::Iso639LanguageDescriptor, DescriptorType::Iso639LanguageDescriptor) => true,
            (DescriptorType::SystemClockDescriptor, DescriptorType::SystemClockDescriptor) => true,
            (DescriptorType::MultiplexBufferUtilizationDescriptor, DescriptorType::MultiplexBufferUtilizationDescriptor) => true,
            (DescriptorType::CopyrightDescriptor, DescriptorType::CopyrightDescriptor) => true,
            (DescriptorType::MaximumBitrateDescriptor, DescriptorType::MaximumBitrateDescriptor) => true,
            (DescriptorType::PrivateDataIndicatorDescriptor, DescriptorType::PrivateDataIndicatorDescriptor) => true,
            (DescriptorType::SmoothingBufferDescriptor, DescriptorType::SmoothingBufferDescriptor) => true,
            (DescriptorType::StdDescriptor, DescriptorType::StdDescriptor) => true,
            (DescriptorType::IbpDescriptor, DescriptorType::IbpDescriptor) => true,
            (DescriptorType::Mpeg4VideoDescriptor, DescriptorType::Mpeg4VideoDescriptor) => true,
            (DescriptorType::Mpeg4AudioDescriptor, DescriptorType::Mpeg4AudioDescriptor) => true,
            (DescriptorType::IodDescriptor, DescriptorType::IodDescriptor) => true,
            (DescriptorType::SlDescriptor, DescriptorType::SlDescriptor) => true,
            (DescriptorType::FmcDescriptor, DescriptorType::FmcDescriptor) => true,
            (DescriptorType::ExternalEsIdDescriptor, DescriptorType::ExternalEsIdDescriptor) => true,
            (DescriptorType::MuxCodeDescriptor, DescriptorType::MuxCodeDescriptor) => true,
            (DescriptorType::FmxBufferSizeDescriptor, DescriptorType::FmxBufferSizeDescriptor) => true,
            (DescriptorType::MultiplexbufferDescriptor, DescriptorType::MultiplexbufferDescriptor) => true,
            (DescriptorType::ContentLabelingDescriptor, DescriptorType::ContentLabelingDescriptor) => true,
            (DescriptorType::MetadataPointerDescriptor, DescriptorType::MetadataPointerDescriptor) => true,
            (DescriptorType::MetadataDescriptor, DescriptorType::MetadataDescriptor) => true,
            (DescriptorType::MetadataStdDescriptor, DescriptorType::MetadataStdDescriptor) => true,
            (DescriptorType::AvcVideoDescriptor, DescriptorType::AvcVideoDescriptor) => true,
            (DescriptorType::AvcTimingAndHrdDescriptor, DescriptorType::AvcTimingAndHrdDescriptor) => true,
            (DescriptorType::Mpeg2AacAudioDescriptor, DescriptorType::Mpeg2AacAudioDescriptor) => true,
            (DescriptorType::FlexMuxTimingDescriptor, DescriptorType::FlexMuxTimingDescriptor) => true,
            (DescriptorType::Mpeg4TextDescriptor, DescriptorType::Mpeg4TextDescriptor) => true,
            (DescriptorType::Mpeg4AudioExtensionDescriptor, DescriptorType::Mpeg4AudioExtensionDescriptor) => true,
            (DescriptorType::AuxiliaryVideoStreamDescriptor, DescriptorType::AuxiliaryVideoStreamDescriptor) => true,
            (DescriptorType::SvcExtensionDescriptor, DescriptorType::SvcExtensionDescriptor) => true,
            (DescriptorType::MvcExtensionDescriptor, DescriptorType::MvcExtensionDescriptor) => true,
            (DescriptorType::J2kVideoDescriptor, DescriptorType::J2kVideoDescriptor) => true,
            (DescriptorType::MvcOperationPointDescriptor, DescriptorType::MvcOperationPointDescriptor) => true,
            (DescriptorType::Mpeg2StereoscopicVideoFormatDescriptor, DescriptorType::Mpeg2StereoscopicVideoFormatDescriptor) => true,
            (DescriptorType::StereoscopicProgramInfoDescriptor, DescriptorType::StereoscopicProgramInfoDescriptor) => true,
            (DescriptorType::StereoscopicVideoInfoDescriptor, DescriptorType::StereoscopicVideoInfoDescriptor) => true,
            (DescriptorType::TransportProfileDescriptor, DescriptorType::TransportProfileDescriptor) => true,
            (DescriptorType::HevcVideoDescriptor, DescriptorType::HevcVideoDescriptor) => true,
            (DescriptorType::ExtensionDescriptor, DescriptorType::ExtensionDescriptor) => true,
            (DescriptorType::UserPrivate, DescriptorType::UserPrivate) => true,
            (DescriptorType::Unknown, DescriptorType::Unknown) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptor_type_equality() {
        assert_eq!(DescriptorType::VideoStreamDescriptor, DescriptorType::VideoStreamDescriptor);
        assert_eq!(DescriptorType::AudioStreamDescriptor, DescriptorType::AudioStreamDescriptor);
        assert_eq!(DescriptorType::HierarchyDescriptor, DescriptorType::HierarchyDescriptor);
        assert_eq!(DescriptorType::RegistrationDescriptor, DescriptorType::RegistrationDescriptor);
        assert_eq!(DescriptorType::DataStreamAlignmentDescriptor, DescriptorType::DataStreamAlignmentDescriptor);
        assert_eq!(DescriptorType::TargetBackgroundGridDescriptor, DescriptorType::TargetBackgroundGridDescriptor);
        assert_eq!(DescriptorType::VideoWindowDescriptor, DescriptorType::VideoWindowDescriptor);
        assert_eq!(DescriptorType::CaDescriptor, DescriptorType::CaDescriptor);
        assert_eq!(DescriptorType::Iso639LanguageDescriptor, DescriptorType::Iso639LanguageDescriptor);
        assert_eq!(DescriptorType::SystemClockDescriptor, DescriptorType::SystemClockDescriptor);
        assert_eq!(DescriptorType::MultiplexBufferUtilizationDescriptor, DescriptorType::MultiplexBufferUtilizationDescriptor);
        assert_eq!(DescriptorType::CopyrightDescriptor, DescriptorType::CopyrightDescriptor);
        assert_eq!(DescriptorType::MaximumBitrateDescriptor, DescriptorType::MaximumBitrateDescriptor);
        assert_eq!(DescriptorType::PrivateDataIndicatorDescriptor, DescriptorType::PrivateDataIndicatorDescriptor);
        assert_eq!(DescriptorType::SmoothingBufferDescriptor, DescriptorType::SmoothingBufferDescriptor);
        assert_eq!(DescriptorType::StdDescriptor, DescriptorType::StdDescriptor);
        assert_eq!(DescriptorType::IbpDescriptor, DescriptorType::IbpDescriptor);
        assert_eq!(DescriptorType::Mpeg4VideoDescriptor, DescriptorType::Mpeg4VideoDescriptor);
        assert_eq!(DescriptorType::Mpeg4AudioDescriptor, DescriptorType::Mpeg4AudioDescriptor);
        assert_eq!(DescriptorType::IodDescriptor, DescriptorType::IodDescriptor);
        assert_eq!(DescriptorType::SlDescriptor, DescriptorType::SlDescriptor);
        assert_eq!(DescriptorType::FmcDescriptor, DescriptorType::FmcDescriptor);
        assert_eq!(DescriptorType::ExternalEsIdDescriptor, DescriptorType::ExternalEsIdDescriptor);
        assert_eq!(DescriptorType::MuxCodeDescriptor, DescriptorType::MuxCodeDescriptor);
        assert_eq!(DescriptorType::FmxBufferSizeDescriptor, DescriptorType::FmxBufferSizeDescriptor);
        assert_eq!(DescriptorType::MultiplexbufferDescriptor, DescriptorType::MultiplexbufferDescriptor);
        assert_eq!(DescriptorType::ContentLabelingDescriptor, DescriptorType::ContentLabelingDescriptor);
        assert_eq!(DescriptorType::MetadataPointerDescriptor, DescriptorType::MetadataPointerDescriptor);
        assert_eq!(DescriptorType::MetadataDescriptor, DescriptorType::MetadataDescriptor);
        assert_eq!(DescriptorType::MetadataStdDescriptor, DescriptorType::MetadataStdDescriptor);
        assert_eq!(DescriptorType::AvcVideoDescriptor, DescriptorType::AvcVideoDescriptor);
        assert_eq!(DescriptorType::Mpeg2AacAudioDescriptor, DescriptorType::Mpeg2AacAudioDescriptor);
        assert_eq!(DescriptorType::FlexMuxTimingDescriptor, DescriptorType::FlexMuxTimingDescriptor);
        assert_eq!(DescriptorType::Mpeg4TextDescriptor, DescriptorType::Mpeg4TextDescriptor);
        assert_eq!(DescriptorType::Mpeg4AudioExtensionDescriptor, DescriptorType::Mpeg4AudioExtensionDescriptor);
        assert_eq!(DescriptorType::AuxiliaryVideoStreamDescriptor, DescriptorType::AuxiliaryVideoStreamDescriptor);
        assert_eq!(DescriptorType::SvcExtensionDescriptor, DescriptorType::SvcExtensionDescriptor);
        assert_eq!(DescriptorType::MvcExtensionDescriptor, DescriptorType::MvcExtensionDescriptor);
        assert_eq!(DescriptorType::J2kVideoDescriptor, DescriptorType::J2kVideoDescriptor);
        assert_eq!(DescriptorType::MvcOperationPointDescriptor, DescriptorType::MvcOperationPointDescriptor);
        assert_eq!(DescriptorType::Mpeg2StereoscopicVideoFormatDescriptor, DescriptorType::Mpeg2StereoscopicVideoFormatDescriptor);
        assert_eq!(DescriptorType::StereoscopicProgramInfoDescriptor, DescriptorType::StereoscopicProgramInfoDescriptor);
        assert_eq!(DescriptorType::StereoscopicVideoInfoDescriptor, DescriptorType::StereoscopicVideoInfoDescriptor);
        assert_eq!(DescriptorType::TransportProfileDescriptor, DescriptorType::TransportProfileDescriptor);
        assert_eq!(DescriptorType::HevcVideoDescriptor, DescriptorType::HevcVideoDescriptor);
        assert_eq!(DescriptorType::ExtensionDescriptor, DescriptorType::ExtensionDescriptor);
        assert_eq!(DescriptorType::UserPrivate, DescriptorType::UserPrivate);
        assert_eq!(DescriptorType::Unknown, DescriptorType::Unknown);
    }

    #[test]
    fn test_descriptor_type_inequality() {
        assert_ne!(DescriptorType::VideoStreamDescriptor, DescriptorType::AudioStreamDescriptor);
        assert_ne!(DescriptorType::HierarchyDescriptor, DescriptorType::RegistrationDescriptor);
        assert_ne!(DescriptorType::DataStreamAlignmentDescriptor, DescriptorType::TargetBackgroundGridDescriptor);
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(DescriptorType::from(0x02), DescriptorType::VideoStreamDescriptor);
        assert_eq!(DescriptorType::from(0x03), DescriptorType::AudioStreamDescriptor);
        assert_eq!(DescriptorType::from(0x04), DescriptorType::HierarchyDescriptor);
        assert_eq!(DescriptorType::from(0x05), DescriptorType::RegistrationDescriptor);
        assert_eq!(DescriptorType::from(0x06), DescriptorType::DataStreamAlignmentDescriptor);
        assert_eq!(DescriptorType::from(0x07), DescriptorType::TargetBackgroundGridDescriptor);
        assert_eq!(DescriptorType::from(0x08), DescriptorType::VideoWindowDescriptor);
        assert_eq!(DescriptorType::from(0x09), DescriptorType::CaDescriptor);
        assert_eq!(DescriptorType::from(0x0A), DescriptorType::Iso639LanguageDescriptor);
        assert_eq!(DescriptorType::from(0x0B), DescriptorType::SystemClockDescriptor);
        assert_eq!(DescriptorType::from(0x0C), DescriptorType::MultiplexBufferUtilizationDescriptor);
        assert_eq!(DescriptorType::from(0x0D), DescriptorType::CopyrightDescriptor);
        assert_eq!(DescriptorType::from(0x0E), DescriptorType::MaximumBitrateDescriptor);
        assert_eq!(DescriptorType::from(0x0F), DescriptorType::PrivateDataIndicatorDescriptor);
        assert_eq!(DescriptorType::from(0x10), DescriptorType::SmoothingBufferDescriptor);
        assert_eq!(DescriptorType::from(0x11), DescriptorType::StdDescriptor);
        assert_eq!(DescriptorType::from(0x12), DescriptorType::IbpDescriptor);
        assert_eq!(DescriptorType::from(0x1B), DescriptorType::Mpeg4VideoDescriptor);
        assert_eq!(DescriptorType::from(0x1C), DescriptorType::Mpeg4AudioDescriptor);
        assert_eq!(DescriptorType::from(0x1D), DescriptorType::IodDescriptor);
        assert_eq!(DescriptorType::from(0x1E), DescriptorType::SlDescriptor);
        assert_eq!(DescriptorType::from(0x1F), DescriptorType::FmcDescriptor);
        assert_eq!(DescriptorType::from(0x20), DescriptorType::ExternalEsIdDescriptor);
        assert_eq!(DescriptorType::from(0x21), DescriptorType::MuxCodeDescriptor);
        assert_eq!(DescriptorType::from(0x22), DescriptorType::FmxBufferSizeDescriptor);
        assert_eq!(DescriptorType::from(0x23), DescriptorType::MultiplexbufferDescriptor);
        assert_eq!(DescriptorType::from(0x24), DescriptorType::ContentLabelingDescriptor);
        assert_eq!(DescriptorType::from(0x25), DescriptorType::MetadataPointerDescriptor);
        assert_eq!(DescriptorType::from(0x26), DescriptorType::MetadataDescriptor);
        assert_eq!(DescriptorType::from(0x27), DescriptorType::MetadataStdDescriptor);
        assert_eq!(DescriptorType::from(0x28), DescriptorType::AvcVideoDescriptor);
        assert_eq!(DescriptorType::from(0x2A), DescriptorType::AvcTimingAndHrdDescriptor);
        assert_eq!(DescriptorType::from(0x2B), DescriptorType::Mpeg2AacAudioDescriptor);
        assert_eq!(DescriptorType::from(0x2C), DescriptorType::FlexMuxTimingDescriptor);
        assert_eq!(DescriptorType::from(0x2D), DescriptorType::Mpeg4TextDescriptor);
        assert_eq!(DescriptorType::from(0x2E), DescriptorType::Mpeg4AudioExtensionDescriptor);
        assert_eq!(DescriptorType::from(0x2F), DescriptorType::AuxiliaryVideoStreamDescriptor);
        assert_eq!(DescriptorType::from(0x30), DescriptorType::SvcExtensionDescriptor);
        assert_eq!(DescriptorType::from(0x31), DescriptorType::MvcExtensionDescriptor);
        assert_eq!(DescriptorType::from(0x32), DescriptorType::J2kVideoDescriptor);
        assert_eq!(DescriptorType::from(0x33), DescriptorType::MvcOperationPointDescriptor);
        assert_eq!(DescriptorType::from(0x34), DescriptorType::Mpeg2StereoscopicVideoFormatDescriptor);
        assert_eq!(DescriptorType::from(0x35), DescriptorType::StereoscopicProgramInfoDescriptor);
        assert_eq!(DescriptorType::from(0x36), DescriptorType::StereoscopicVideoInfoDescriptor);
        assert_eq!(DescriptorType::from(0x37), DescriptorType::TransportProfileDescriptor);
        assert_eq!(DescriptorType::from(0x38), DescriptorType::HevcVideoDescriptor);
        assert_eq!(DescriptorType::from(0x3F), DescriptorType::ExtensionDescriptor);
        assert_eq!(DescriptorType::from(0x40), DescriptorType::UserPrivate);
        assert_eq!(DescriptorType::from(0xFF), DescriptorType::UserPrivate);
        assert_eq!(DescriptorType::from(0x00), DescriptorType::Unknown);
    }
}