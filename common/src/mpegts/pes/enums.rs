use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum StreamType {
    ProgramStreamMap,
    PrivateStream1,
    PaddingStream,
    PrivateStream2,
    AudioStream(u8),
    VideoStream(u8),
    ECMStream,
    EMMStream,
    DSMCCStream,
    ISOIEC13522Stream,
    H2221TypeA,
    H2221TypeB,
    H2221TypeC,
    H2221TypeD,
    H2221TypeE,
    AncillaryStream,
    SLPacketizedStream,
    FlexMuxStream,
    MetadataStream,
    ExtendedStreamId,
    ReservedDataStream,
    ProgramStreamDirectory,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TrickModeControlValues {
    FastForward,
    SlowMotion,
    FreezeFrame,
    FastReverse,
    SlowReverse,
    Reserved,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PtsDtsFlags {
    No,
    Forbidden,
    PresentPts,
    PresentPtsAndDts,
}

impl From<u8> for PtsDtsFlags {
    fn from(value: u8) -> Self {
        match value {
            0b00 => PtsDtsFlags::No,
            0b01 => PtsDtsFlags::Forbidden,
            0b10 => PtsDtsFlags::PresentPts,
            0b11 => PtsDtsFlags::PresentPtsAndDts,
            _ => PtsDtsFlags::Forbidden,
        }
    }
}

impl From<PtsDtsFlags> for u8 {
    fn from(val: PtsDtsFlags) -> Self {
        match val {
            PtsDtsFlags::No => 0b00,
            PtsDtsFlags::Forbidden => 0b01,
            PtsDtsFlags::PresentPts => 0b10,
            PtsDtsFlags::PresentPtsAndDts => 0b11,
        }
    }
}

impl From<u8> for TrickModeControlValues {
    fn from(value: u8) -> Self {
        match value {
            0b000 => TrickModeControlValues::FastForward,
            0b001 => TrickModeControlValues::SlowMotion,
            0b010 => TrickModeControlValues::FreezeFrame,
            0b011 => TrickModeControlValues::FastReverse,
            0b100 => TrickModeControlValues::SlowReverse,
            _ => TrickModeControlValues::Reserved,
        }
    }
}

impl From<TrickModeControlValues> for u8 {
    fn from(val: TrickModeControlValues) -> Self {
        match val {
            TrickModeControlValues::FastForward => 0b000,
            TrickModeControlValues::SlowMotion => 0b001,
            TrickModeControlValues::FreezeFrame => 0b010,
            TrickModeControlValues::FastReverse => 0b011,
            TrickModeControlValues::SlowReverse => 0b100,
            TrickModeControlValues::Reserved => 0b111,
        }
    }
}

impl From<u8> for StreamType {
    fn from(stream_id: u8) -> Self {
        match stream_id {
            0xBC => StreamType::ProgramStreamMap,
            0xBD => StreamType::PrivateStream1,
            0xBE => StreamType::PaddingStream,
            0xBF => StreamType::PrivateStream2,
            0xF0 => StreamType::ECMStream,
            0xF1 => StreamType::EMMStream,
            0xF2 => StreamType::DSMCCStream,
            0xF3 => StreamType::ISOIEC13522Stream,
            0xF4 => StreamType::H2221TypeA,
            0xF5 => StreamType::H2221TypeB,
            0xF6 => StreamType::H2221TypeC,
            0xF7 => StreamType::H2221TypeD,
            0xF8 => StreamType::H2221TypeE,
            0xF9 => StreamType::AncillaryStream,
            0xFA => StreamType::SLPacketizedStream,
            0xFB => StreamType::FlexMuxStream,
            0xFC => StreamType::MetadataStream,
            0xFD => StreamType::ExtendedStreamId,
            0xFE => StreamType::ReservedDataStream,
            0xFF => StreamType::ProgramStreamDirectory,
            id @ 0xC0..=0xDF => StreamType::AudioStream(id & 0x1F),
            id @ 0xE0..=0xEF => StreamType::VideoStream(id & 0x0F),
            _ => StreamType::Unknown,
        }
    }
}
