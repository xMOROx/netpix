use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Packets,
    RtpSection(RtpSection),
    MpegTsSection(MpegTsSection),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RtpSection {
    Packets,
    RtcpPackets,
    Streams,
    Plot,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MpegTsSection {
    Packets,
    Streams,
    Information,
    Plot,
}

impl Tab {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Packets,
            Self::RtpSection(RtpSection::Packets),
            Self::RtpSection(RtpSection::RtcpPackets),
            Self::RtpSection(RtpSection::Streams),
            Self::RtpSection(RtpSection::Plot),
            Self::MpegTsSection(MpegTsSection::Packets),
            Self::MpegTsSection(MpegTsSection::Streams),
            Self::MpegTsSection(MpegTsSection::Information),
            Self::MpegTsSection(MpegTsSection::Plot),
        ]
    }
    
    pub fn general_sections() -> Vec<Self> {
        vec![
            Self::Packets,
        ]
    }

    pub fn rtp_sections() -> Vec<Self> {
        vec![
            Self::RtpSection(RtpSection::Packets),
            Self::RtpSection(RtpSection::RtcpPackets),
            Self::RtpSection(RtpSection::Streams),
            Self::RtpSection(RtpSection::Plot),
        ]
    }

    pub fn mpeg_ts_sections() -> Vec<Self> {
        vec![
            Self::MpegTsSection(MpegTsSection::Packets),
            Self::MpegTsSection(MpegTsSection::Streams),
            Self::MpegTsSection(MpegTsSection::Information),
            Self::MpegTsSection(MpegTsSection::Plot),
        ]
    }

    pub fn from_string(tab_str: String) -> Option<Self> {
        Tab::all()
            .into_iter()
            .find(|tab| tab_str == tab.to_string())
    }
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            Self::Packets => "📦 All Packets",
            Self::RtpSection(section) => match section {
                RtpSection::Packets => "🔈RTP Packets",
                RtpSection::RtcpPackets => "📃 RTCP Packets",
                RtpSection::Streams => "🔴 RTP Streams",
                RtpSection::Plot => "📈 RTP Plot",
            },
            Self::MpegTsSection(section) => match section {
                MpegTsSection::Packets => "📺 MPEG-TS Packets",
                MpegTsSection::Streams => "🎥 MPEG-TS Streams",
                MpegTsSection::Information => "ℹ️ MPEG-TS Info",
                MpegTsSection::Plot => "📊 MPEG-TS Plot",
            },
        };

        write!(f, "{}", ret)
    }
}
