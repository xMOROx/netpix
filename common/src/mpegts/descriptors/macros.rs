#[macro_export]
macro_rules! declare_descriptor_variants {
    ($(($variant:ident, $type:ty)),* $(,)?) => {
        #[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
        pub enum Descriptors {
            $($variant($type),)*
            UserPrivate(u8),
            Unknown,
        }
    };
}

#[macro_export]
macro_rules! impl_descriptor_display {
    ($(($variant:ident)),* $(,)?) => {
        impl std::fmt::Display for Descriptors {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Descriptors::$variant(descriptor) => write!(f, "{}", descriptor),)*
                    Descriptors::UserPrivate(data) => write!(f, "User Private: {}", data),
                    Descriptors::Unknown => write!(f, "Unknown"),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_descriptor_partial_eq {
    ($(($variant:ident)),* $(,)?) => {
        impl PartialEq for Descriptors {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (Descriptors::$variant(a), Descriptors::$variant(b)) => a == b,
                    )*
                    (Descriptors::UserPrivate(a), Descriptors::UserPrivate(b)) => a == b,
                    (Descriptors::Unknown, Descriptors::Unknown) => true,
                    _ => false,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_descriptor_unmarshall_match {
    ($(($variant:ident, $tag:ident, $type:ty)),* $(,)?) => {
        impl Descriptors {
            pub fn unmarshall(data: &[u8]) -> Option<Self> {
                let header = DescriptorHeader::unmarshall(data);
                let payload = &data[2..];
                match header.descriptor_tag {
                    $(
                        DescriptorTag::$tag => {
                            <$type>::unmarshall(header, payload)
                                .map(Descriptors::$variant)
                        }
                    )*
                    DescriptorTag::UserPrivate => Some(Descriptors::UserPrivate(data[0])),
                    _ => Some(Descriptors::Unknown),
                }
            }
            pub fn unmarshall_many(data: &[u8]) -> Vec<Self> {
                let mut descriptors = Vec::new();
                let mut offset = 0;
                while offset < data.len() {
                    let header = DescriptorHeader::unmarshall(&data[offset..]);
                    if let Some(descriptor) = Self::unmarshall(
                        &data[offset..(header.descriptor_length + HEADER_SIZE) as usize + offset]
                    ) {
                        descriptors.push(descriptor);
                    }
                    offset += header.descriptor_length as usize + HEADER_SIZE as usize;
                }
                descriptors
            }
        }
    };
}
