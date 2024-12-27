#[cfg(test)]
mod tests;

use bincode::{Decode, Encode};
use std::fmt::Display;

#[derive(Encode, PartialEq, Eq, Decode, Debug, Clone, Copy, Ord, PartialOrd, Hash)]
pub enum StreamType {
    Reserved,
    Video111722,
    Video138182,
    Audio111723,
    Audio138183,
    PrivateSections,
    PESPackets,
    MHEG,
    DSMCC,
    H2221,
    TypeA,
    TypeB,
    TypeC,
    TypeD,
    Auxiliary,
    AudioADTS,
    Visual,
    AudioLATM,
    SLpacketizedstreamorFlexMuxstreamPESPacketized,
    SLpacketizedstreamorFlexMuxstreamSectionPacketized,
    Download,
    MetadataPES,
    MetadataSections,
    MetadataCarousel,
    MetadataObject,
    MetadataDownload,
    IPMP,
    VideoH264,
    RawAudio,
    Text,
    AuxiliaryVideo,
    VideoSVC,
    VideoMVC,
    ConformingVideoStream,
    Video3DH264,
    VideoHEVC,
    VideoHEVCTemporal,
    VideoMVCD,
    Timeline,
    VideoHEVCEnhanceG,
    VideoHEVCEnhanceGTemp,
    VideoHEVCEnhanceH,
    VideoHEVCEnhanceHTemp,
    GreenUnits,
    AudioMHASMain,
    AudioMHASAux,
    QualityUnits,
    Reserved138181,
    IPMPStream,
    UserPrivate,
}

impl From<u8> for StreamType {
    fn from(stream_type: u8) -> Self {
        match stream_type {
            0x00 => StreamType::Reserved,
            0x01 => StreamType::Video111722,
            0x02 => StreamType::Video138182,
            0x03 => StreamType::Audio111723,
            0x04 => StreamType::Audio138183,
            0x05 => StreamType::PrivateSections,
            0x06 => StreamType::PESPackets,
            0x07 => StreamType::MHEG,
            0x08 => StreamType::DSMCC,
            0x09 => StreamType::H2221,
            0x0A => StreamType::TypeA,
            0x0B => StreamType::TypeB,
            0x0C => StreamType::TypeC,
            0x0D => StreamType::TypeD,
            0x0E => StreamType::Auxiliary,
            0x0F => StreamType::AudioADTS,
            0x10 => StreamType::Visual,
            0x11 => StreamType::AudioLATM,
            0x12 => StreamType::SLpacketizedstreamorFlexMuxstreamPESPacketized,
            0x13 => StreamType::SLpacketizedstreamorFlexMuxstreamSectionPacketized,
            0x14 => StreamType::Download,
            0x15 => StreamType::MetadataPES,
            0x16 => StreamType::MetadataSections,
            0x17 => StreamType::MetadataCarousel,
            0x18 => StreamType::MetadataObject,
            0x19 => StreamType::MetadataDownload,
            0x1A => StreamType::IPMP,
            0x1B => StreamType::VideoH264,
            0x1C => StreamType::RawAudio,
            0x1D => StreamType::Text,
            0x1E => StreamType::AuxiliaryVideo,
            0x1F => StreamType::VideoSVC,
            0x20 => StreamType::VideoMVC,
            0x21 => StreamType::ConformingVideoStream,
            0x22 => StreamType::Video3DH264,
            0x23 => StreamType::VideoHEVC,
            0x24 => StreamType::VideoHEVCTemporal,
            0x25 => StreamType::VideoMVCD,
            0x26 => StreamType::Timeline,
            0x27 => StreamType::VideoHEVCEnhanceG,
            0x28 => StreamType::VideoHEVCEnhanceGTemp,
            0x29 => StreamType::VideoHEVCEnhanceH,
            0x2A => StreamType::VideoHEVCEnhanceHTemp,
            0x2B => StreamType::GreenUnits,
            0x2C => StreamType::AudioMHASMain,
            0x2D => StreamType::AudioMHASAux,
            0x2E => StreamType::QualityUnits,
            0x30..=0x7E => StreamType::Reserved138181,
            0x7F => StreamType::IPMPStream,
            _ => StreamType::UserPrivate,
        }
    }
}

impl Display for StreamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StreamType::Reserved => "Reserved",
            StreamType::Video111722 => "MPEG-1 Video",
            StreamType::Video138182 => "MPEG-2 Video",
            StreamType::Audio111723 => "MPEG-1 Audio",
            StreamType::Audio138183 => "MPEG-2 Audio",
            StreamType::PrivateSections => "Private Sections",
            StreamType::PESPackets => "PES Packets",
            StreamType::MHEG => "MHEG",
            StreamType::DSMCC => "DSM-CC",
            StreamType::H2221 => "H.222.1",
            StreamType::TypeA => "Type A",
            StreamType::TypeB => "Type B",
            StreamType::TypeC => "Type C",
            StreamType::TypeD => "Type D",
            StreamType::Auxiliary => "Auxiliary",
            StreamType::AudioADTS => "ADTS Audio",
            StreamType::Visual => "MPEG-4 Visual",
            StreamType::AudioLATM => "LATM Audio",
            StreamType::SLpacketizedstreamorFlexMuxstreamPESPacketized => "SL/FlexMux in PES",
            StreamType::SLpacketizedstreamorFlexMuxstreamSectionPacketized => {
                "SL/FlexMux in Sections"
            }
            StreamType::Download => "Synchronized Download Protocol",
            StreamType::MetadataPES => "Metadata PES",
            StreamType::MetadataSections => "Metadata Sections",
            StreamType::MetadataCarousel => "Metadata Carousel",
            StreamType::MetadataObject => "Metadata Object",
            StreamType::MetadataDownload => "Metadata Synchronized Download Protocol",
            StreamType::IPMP => "IPMP",
            StreamType::VideoH264 => "H.264 Video",
            StreamType::RawAudio => "Raw Audio",
            StreamType::Text => "Text",
            StreamType::AuxiliaryVideo => "Auxiliary Video",
            StreamType::VideoSVC => "SVC Video",
            StreamType::VideoMVC => "MVC Video",
            StreamType::ConformingVideoStream => "Conforming Video Stream",
            StreamType::Video3DH264 => "3D H.264 Video",
            StreamType::VideoHEVC => "HEVC Video",
            StreamType::VideoHEVCTemporal => "HEVC Temporal Video",
            StreamType::VideoMVCD => "MVCD  Video",
            StreamType::Timeline => "Timeline",
            StreamType::VideoHEVCEnhanceG => "HEVC Enhancement G  Video",
            StreamType::VideoHEVCEnhanceGTemp => "HEVC Temporal G  Video",
            StreamType::VideoHEVCEnhanceH => "HEVC Enhancement H  Video",
            StreamType::VideoHEVCEnhanceHTemp => "HEVC Temporal H  Video",
            StreamType::GreenUnits => "Green Units",
            StreamType::AudioMHASMain => "MHAS Main Audio",
            StreamType::AudioMHASAux => "MHAS Aux Audio",
            StreamType::QualityUnits => "Quality Units",
            StreamType::Reserved138181 => "Reserved",
            StreamType::IPMPStream => "IPMP Stream",
            StreamType::UserPrivate => "User Private",
        };
        write!(f, "{}", str)
    }
}

pub fn stream_type_into_unique_letter(stream_type: &StreamType) -> &str {
    match stream_type {
        StreamType::Reserved => "R",
        StreamType::Video111722 => "V1",
        StreamType::Video138182 => "V2",
        StreamType::Audio111723 => "A1",
        StreamType::Audio138183 => "A2",
        StreamType::PrivateSections => "PS",
        StreamType::PESPackets => "PE",
        StreamType::MHEG => "M",
        StreamType::DSMCC => "D",
        StreamType::H2221 => "H",
        StreamType::TypeA => "TA",
        StreamType::TypeB => "TB",
        StreamType::TypeC => "TC",
        StreamType::TypeD => "TD",
        StreamType::Auxiliary => "AU",
        StreamType::AudioADTS => "AA",
        StreamType::Visual => "V",
        StreamType::AudioLATM => "AL",
        StreamType::SLpacketizedstreamorFlexMuxstreamPESPacketized => "SLP",
        StreamType::SLpacketizedstreamorFlexMuxstreamSectionPacketized => "SLS",
        StreamType::Download => "D",
        StreamType::MetadataPES => "MP",
        StreamType::MetadataSections => "MS",
        StreamType::MetadataCarousel => "MC",
        StreamType::MetadataObject => "MO",
        StreamType::MetadataDownload => "MD",
        StreamType::IPMP => "I",
        StreamType::VideoH264 => "VH",
        StreamType::RawAudio => "RA",
        StreamType::Text => "T",
        StreamType::AuxiliaryVideo => "AV",
        StreamType::VideoSVC => "VS",
        StreamType::VideoMVC => "VM",
        StreamType::ConformingVideoStream => "CV",
        StreamType::Video3DH264 => "3D",
        StreamType::VideoHEVC => "VH",
        StreamType::VideoHEVCTemporal => "VT",
        StreamType::VideoMVCD => "MV",
        StreamType::Timeline => "T",
        StreamType::VideoHEVCEnhanceG => "VG",
        StreamType::VideoHEVCEnhanceGTemp => "VGT",
        StreamType::VideoHEVCEnhanceH => "VH",
        StreamType::VideoHEVCEnhanceHTemp => "VHT",
        StreamType::GreenUnits => "G",
        StreamType::AudioMHASMain => "AM",
        StreamType::AudioMHASAux => "AA",
        StreamType::QualityUnits => "Q",
        StreamType::Reserved138181 => "R",
        StreamType::IPMPStream => "I",
        StreamType::UserPrivate => "UP",
    }
}

pub fn get_stream_type_category(stream_type: &StreamType) -> &str {
    match stream_type {
        StreamType::Video111722
        | StreamType::Video138182
        | StreamType::Visual
        | StreamType::VideoH264
        | StreamType::VideoSVC
        | StreamType::VideoMVC
        | StreamType::Video3DH264
        | StreamType::VideoHEVC
        | StreamType::VideoHEVCTemporal
        | StreamType::VideoMVCD
        | StreamType::VideoHEVCEnhanceG
        | StreamType::VideoHEVCEnhanceGTemp
        | StreamType::VideoHEVCEnhanceH
        | StreamType::VideoHEVCEnhanceHTemp
        | StreamType::AuxiliaryVideo => "Video",

        StreamType::Audio111723
        | StreamType::Audio138183
        | StreamType::AudioADTS
        | StreamType::AudioLATM
        | StreamType::RawAudio
        | StreamType::AudioMHASMain
        | StreamType::AudioMHASAux => "Audio",

        _ => "Other",
    }
}
