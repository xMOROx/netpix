use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
pub struct RawPayload {
    pub data: Vec<u8>,
}
