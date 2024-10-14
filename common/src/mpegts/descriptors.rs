mod types;
mod video_stream;
mod audio_stream;
mod hierarchy;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::audio_stream::AudioStreamDescriptor;
use crate::mpegts::descriptors::hierarchy::HierarchyDescriptor;
use crate::mpegts::descriptors::types::DescriptorType;
use crate::mpegts::descriptors::video_stream::VideoStreamDescriptor;

pub trait ParsableDescriptor<T>: Debug {
    fn descriptor_tag(&self) -> u8;
    fn descriptor_length(&self) -> u8;
    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<T>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub enum Descriptor {
    VideoStreamDescriptor(VideoStreamDescriptor),
    AudioStreamDescriptor(AudioStreamDescriptor),
    HierarchyDescriptor(HierarchyDescriptor),
}

impl Descriptor {
    pub fn unmarshall(data: &[u8]) -> Option<Self> {
        let header = DescriptorHeader::unmarshall(data);
        let payload = &data[2..];
        match header.descriptor_tag {
            DescriptorType::VideoStreamDescriptor => {
                VideoStreamDescriptor::unmarshall(header, payload).map(|descriptor| {
                    Descriptor::VideoStreamDescriptor(descriptor)
                })
            },
            DescriptorType::AudioStreamDescriptor => {
                AudioStreamDescriptor::unmarshall(header, payload).map(|descriptor| {
                    Descriptor::AudioStreamDescriptor(descriptor)
                })
            },
            DescriptorType::HierarchyDescriptor => {
                HierarchyDescriptor::unmarshall(header, payload).map(|descriptor| {
                    Descriptor::HierarchyDescriptor(descriptor)
                })
            },

            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct DescriptorHeader {
    pub descriptor_tag: DescriptorType,
    pub descriptor_length: u8,
}

impl DescriptorHeader {
    pub fn unmarshall(data: &[u8]) -> Self {
        let descriptor_tag = DescriptorType::from(data[0]);
        let descriptor_length = data[1];

        DescriptorHeader {
            descriptor_tag,
            descriptor_length,
        }
    }
}

impl PartialEq for Descriptor {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq for DescriptorHeader {
    fn eq(&self, other: &Self) -> bool {
        let descriptor_tag = self.descriptor_tag == other.descriptor_tag;
        let descriptor_length = self.descriptor_length == other.descriptor_length;

        descriptor_tag && descriptor_length
    }
}