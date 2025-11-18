use crate::app::common::PlotRegistry;
use crate::app::plots::RtpStreamsPlot;
use crate::{
    app::{
        common::table::{TableBase, TableRegistry},
        tab::{MpegTsSection, RtpSection},
    },
    streams::RefStreams,
};
pub use common::*;
pub use constants::*;
use eframe::egui;
use egui::{ComboBox, Label, TextWrapMode, Ui, Widget};
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
pub use filter_utils::*;
use log::{error, warn};
use netpix_common::{MpegtsStreamKey, Request, Response, RtpStreamKey, Source};
use std::{collections::HashMap, sync::Arc};
use tab::Tab;
use tables::{
    MpegTsInformationTable, MpegTsPacketsTable, MpegTsStreamsTable, PacketsTable, RtcpPacketsTable,
    RtpPacketsTable, RtpStreamsTable,
};
pub use types::App;
pub use utils::*;

mod common;
mod constants;
mod filter_utils;
mod plots;
mod tab;
mod tables;
mod types;
mod ui_components;
mod utils;
