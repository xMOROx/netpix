use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Packets,
    RtpSection(RtpSection),
    MpegTsSection(MpegTsSection),
}

impl Tab {
    pub fn display_name(&self) -> String {
        self.to_string()
    }
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
        vec![Self::Packets]
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
        match self {
            Self::Packets => write!(f, "📦 Packets"),
            Self::RtpSection(section) => section.fmt(f),
            Self::MpegTsSection(section) => section.fmt(f),
        }
    }
}

impl fmt::Display for RtpSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            Self::Packets => "🔈RTP Packets",
            Self::RtcpPackets => "📃 RTCP Packets",
            Self::Streams => "🔴 RTP Streams",
            Self::Plot => "📈 RTP Plot",
        };

        write!(f, "{}", ret)
    }
}

impl fmt::Display for MpegTsSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            Self::Packets => "📺 MPEG-TS Packets",
            Self::Streams => "🎥 MPEG-TS Streams",
            Self::Information => "ℹ️ MPEG-TS Info",
            Self::Plot => "📊 MPEG-TS Plot",
        };

        write!(f, "{}", ret)
    }
}
