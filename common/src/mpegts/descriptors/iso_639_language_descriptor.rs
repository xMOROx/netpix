use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const SECTION_LENGTH: u8 = 4;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct Iso639LanguageDescriptor {
    pub header: DescriptorHeader,
    pub section: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct Section {
    pub language_code: String,
    pub audio_type: AudioType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub enum AudioType {
    Undefined,
    CleanEffects,
    HearingImpaired,
    VisualImpairedCommentary,
    UserPrivate,
    Reserved,
}

impl ParsableDescriptor<Iso639LanguageDescriptor> for Iso639LanguageDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<Iso639LanguageDescriptor> {
        if data.len() < 4 {
            return None;
        }

        let reader = BitReader::new(data);
        let number_of_sections = data.len() as u8 / SECTION_LENGTH;
        let mut section = Vec::new();

        for i in 0..number_of_sections {
            let start = i as usize * SECTION_LENGTH as usize;
            let lang_bytes = reader.get_bytes(start, 3)?;
            let language_code = String::from_utf8(lang_bytes).ok()?;
            let audio_type = AudioType::from(data[start + 3]);

            section.push(Section {
                language_code,
                audio_type,
            });
        }

        Some(Iso639LanguageDescriptor { header, section })
    }
}

impl std::fmt::Display for Iso639LanguageDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sections = String::new();
        for s in &self.section {
            sections.push_str(&format!(
                "Language Code: {}\nAudio Type: {:?}\n",
                s.language_code, s.audio_type
            ));
        }
        write!(f, "{}", sections)
    }
}

impl PartialEq for Iso639LanguageDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.section == other.section
    }
}

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.language_code == other.language_code && self.audio_type == other.audio_type
    }
}

impl PartialEq for AudioType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (AudioType::Undefined, AudioType::Undefined)
                | (AudioType::CleanEffects, AudioType::CleanEffects)
                | (AudioType::HearingImpaired, AudioType::HearingImpaired)
                | (
                    AudioType::VisualImpairedCommentary,
                    AudioType::VisualImpairedCommentary
                )
                | (AudioType::UserPrivate, AudioType::UserPrivate)
                | (AudioType::Reserved, AudioType::Reserved)
        )
    }
}

impl From<u8> for AudioType {
    fn from(value: u8) -> Self {
        match value {
            0x0 => AudioType::Undefined,
            0x1 => AudioType::CleanEffects,
            0x2 => AudioType::HearingImpaired,
            0x3 => AudioType::VisualImpairedCommentary,
            0x04..=0x7F => AudioType::UserPrivate,
            0x80..=0xFF => AudioType::Reserved,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

    #[test]
    fn test_iso_639_language_descriptor_unmarshall() {
        let data = vec![
            b'e', b'n', b'g', 0x01, // English, CleanEffects
            b's', b'p', b'a', 0x02, // Spanish, HearingImpaired
        ];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0A),
            descriptor_length: data.len() as u8,
        };
        let descriptor = Iso639LanguageDescriptor {
            header: header.clone(),
            section: vec![
                Section {
                    language_code: "eng".to_string(),
                    audio_type: AudioType::CleanEffects,
                },
                Section {
                    language_code: "spa".to_string(),
                    audio_type: AudioType::HearingImpaired,
                },
            ],
        };

        assert_eq!(
            Iso639LanguageDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
    }

    #[test]
    fn test_iso_639_language_descriptor_unmarshall_invalid_length() {
        let data = vec![b'e', b'n', b'g', 0x01]; // Only one section
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0A),
            descriptor_length: (data.len() - 1) as u8, // Invalid length
        };

        assert_eq!(
            Iso639LanguageDescriptor::unmarshall(header, &data[1..]),
            None
        );
    }

    #[test]
    fn test_audio_type_from() {
        assert_eq!(AudioType::from(0), AudioType::Undefined);
        assert_eq!(AudioType::from(1), AudioType::CleanEffects);
        assert_eq!(AudioType::from(2), AudioType::HearingImpaired);
        assert_eq!(AudioType::from(3), AudioType::VisualImpairedCommentary);
        assert_eq!(AudioType::from(4), AudioType::UserPrivate);
        assert_eq!(AudioType::from(128), AudioType::Reserved);
    }

    #[test]
    fn test_iso_639_language_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0A),
            descriptor_length: 8,
        };
        let descriptor1 = Iso639LanguageDescriptor {
            header: header.clone(),
            section: vec![
                Section {
                    language_code: "eng".to_string(),
                    audio_type: AudioType::CleanEffects,
                },
                Section {
                    language_code: "spa".to_string(),
                    audio_type: AudioType::HearingImpaired,
                },
            ],
        };
        let descriptor2 = Iso639LanguageDescriptor {
            header,
            section: vec![
                Section {
                    language_code: "eng".to_string(),
                    audio_type: AudioType::CleanEffects,
                },
                Section {
                    language_code: "spa".to_string(),
                    audio_type: AudioType::HearingImpaired,
                },
            ],
        };

        assert_eq!(descriptor1, descriptor2);
    }
}
