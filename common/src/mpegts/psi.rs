use serde::{Deserialize, Serialize};

pub mod constants;
pub mod pat;
pub mod pmt;
pub mod psi_buffer;
#[cfg(test)]
mod tests;

pub trait ProgramSpecificInformation {
    fn get_header(&self) -> &ProgramSpecificInformationHeader;
    fn get_table_id(&self) -> TableId;
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct ProgramSpecificInformationHeader {
    pub table_id: u8,
    pub section_syntax_indicator: bool,
    pub section_length: u16,
    pub version_number: u8,
    pub current_next_indicator: bool,
    pub section_number: u8,
    pub last_section_number: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum PsiTypes {
    PAT(u16), //transport_stream_id
    PMT(u16), // pid value
    NONE,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub enum TableId {
    ProgramAssociationSection,
    ConditionalAccessSection,
    TsProgramMapSection,
    TsDescriptionSection,
    IsoIec14496SceneDescriptionSection,
    IsoIec14496ObjectDescriptorSection,
    MetadataSection,
    IpmpControlInformationSection,
    IsoIec14496Section,
    IsoIse23001_11GreenAccessUnitSection,
    IsoIse23001_10QualityAccessUnitSection,
    RecItuTH222_0IsoIec13818_1Reserved,
    DefinedInIsoIec13818_6,
    UserPrivate,
    Forbidden,
}

impl PartialEq for ProgramSpecificInformationHeader {
    fn eq(&self, other: &Self) -> bool {
        let table_id = self.table_id == other.table_id;
        let section_syntax_indicator =
            self.section_syntax_indicator == other.section_syntax_indicator;
        let section_length = self.section_length == other.section_length;
        let version_number = self.version_number == other.version_number;
        let current_next_indicator = self.current_next_indicator == other.current_next_indicator;
        let section_number = self.section_number == other.section_number;
        let last_section_number = self.last_section_number == other.last_section_number;

        table_id
            && section_syntax_indicator
            && section_length
            && version_number
            && current_next_indicator
            && section_number
            && last_section_number
    }
}

impl ProgramSpecificInformation for ProgramSpecificInformationHeader {
    fn get_header(&self) -> &ProgramSpecificInformationHeader {
        self
    }

    fn get_table_id(&self) -> TableId {
        self.table_id.into()
    }
}

impl From<u8> for TableId {
    fn from(table_id: u8) -> Self {
        match table_id {
            0x00 => TableId::ProgramAssociationSection,
            0x01 => TableId::ConditionalAccessSection,
            0x02 => TableId::TsProgramMapSection,
            0x03 => TableId::TsDescriptionSection,
            0x04 => TableId::IsoIec14496SceneDescriptionSection,
            0x05 => TableId::IsoIec14496ObjectDescriptorSection,
            0x06 => TableId::MetadataSection,
            0x07 => TableId::IpmpControlInformationSection,
            0x08 => TableId::IsoIec14496Section,
            0x09 => TableId::IsoIse23001_11GreenAccessUnitSection,
            0x0A => TableId::IsoIse23001_10QualityAccessUnitSection,
            0x0B..=0x37 => TableId::RecItuTH222_0IsoIec13818_1Reserved,
            0x38..=0x3F => TableId::DefinedInIsoIec13818_6,
            0x40..=0xFE => TableId::UserPrivate,
            _ => TableId::Forbidden,
        }
    }
}
