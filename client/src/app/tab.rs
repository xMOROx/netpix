use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Packets,
    RtpPackets,
    RtcpPackets,
    RtpStreams,
    RtpPlot,
    MpegTsPackets,
    MpegTsStreams,
    MpegTsInformations,
    MpegTsPlot,
}

impl Tab {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Packets,
            Self::RtpPackets,
            Self::RtcpPackets,
            Self::RtpStreams,
            Self::RtpPlot,
            Self::MpegTsPackets,
            Self::MpegTsStreams,
            Self::MpegTsInformations,
            Self::MpegTsPlot,
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
            Self::RtpPackets => "🔈RTP Packets",
            Self::RtcpPackets => "📃 RTCP Packets",
            Self::RtpStreams => "🔴 RTP Streams",
            Self::RtpPlot => "📈 RTP Plot",
            Self::MpegTsPackets => "📺 MPEG-TS Packets",
            Self::MpegTsStreams => "🎥 MPEG-TS Streams",
            Self::MpegTsInformations => "ℹ️ MPEG-TS Info",
            Self::MpegTsPlot => "📊 MPEG-TS Plot",
        };

        write!(f, "{}", ret)
    }
}
