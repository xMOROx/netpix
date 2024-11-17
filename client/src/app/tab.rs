use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Packets,
    RtpSection(RtpSection),
    MpegTsSection(MpegTsSection),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RtpSection {
    RtpPackets,
    RtcpPackets,
    RtpStreams,
    RtpPlot,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MpegTsSection {
    MpegTsPackets,
    MpegTsStreams,
    MpegTsInformations,
    MpegTsPlot,
}

impl Tab {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Packets,
            Self::RtpSection(RtpSection::RtpPackets),
            Self::RtpSection(RtpSection::RtcpPackets),
            Self::RtpSection(RtpSection::RtpStreams),
            Self::RtpSection(RtpSection::RtpPlot),
            Self::MpegTsSection(MpegTsSection::MpegTsPackets),
            Self::MpegTsSection(MpegTsSection::MpegTsStreams),
            Self::MpegTsSection(MpegTsSection::MpegTsInformations),
            Self::MpegTsSection(MpegTsSection::MpegTsPlot),
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
                RtpSection::RtpPackets => "🔈RTP Packets",
                RtpSection::RtcpPackets => "📃 RTCP Packets",
                RtpSection::RtpStreams => "🔴 RTP Streams",
                RtpSection::RtpPlot => "📈 RTP Plot",
            },
            Self::MpegTsSection(section) => match section {
                MpegTsSection::MpegTsPackets => "📺 MPEG-TS Packets",
                MpegTsSection::MpegTsStreams => "🎥 MPEG-TS Streams",
                MpegTsSection::MpegTsInformations => "ℹ️ MPEG-TS Info",
                MpegTsSection::MpegTsPlot => "📊 MPEG-TS Plot",
            },
        };

        write!(f, "{}", ret)
    }
}
