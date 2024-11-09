use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct AdaptationField {
    pub adaptation_field_length: u8,
    // discontntinuity_indicator: Option<bool>,

}