use serde::{Deserialize, Serialize};

use super::enums::TrickModeControlValues;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct TrickModeControl {
    pub trick_mode_control: TrickModeControlValues,
    pub field_id: Option<u8>,
    pub intra_slice_refresh: Option<u8>,
    pub frequency_truncation: Option<u8>,
    pub rep_cntrl: Option<u8>,
}

impl TrickModeControl {
    pub fn build(data: &[u8]) -> Option<Self> {
        Self::unmarshall(data)
    }

    fn unmarshall(data: &[u8]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let trick_mode_control = TrickModeControlValues::from((data[0] & 0b11100000) >> 5);
        let mut field_id = None;
        let mut intra_slice_refresh = None;
        let mut frequency_truncation = None;
        let mut rep_cntrl = None;

        match trick_mode_control {
            TrickModeControlValues::FastForward | TrickModeControlValues::FastReverse => {
                field_id = Some((data[0] & 0b00011000) >> 3);
                intra_slice_refresh = Some((data[0] & 0b00000100) >> 2);
                frequency_truncation = Some(data[0] & 0b00000011);
            }
            TrickModeControlValues::SlowMotion | TrickModeControlValues::SlowReverse => {
                rep_cntrl = Some(data[0] & 0b00011111);
            }
            TrickModeControlValues::FreezeFrame => {
                field_id = Some((data[0] & 0b00011000) >> 3);
            }
            TrickModeControlValues::Reserved => {}
        }

        Some(Self {
            trick_mode_control,
            field_id,
            intra_slice_refresh,
            frequency_truncation,
            rep_cntrl,
        })
    }
}
