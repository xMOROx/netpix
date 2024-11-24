use super::BitReader;

pub struct PesExtensionReader<'a> {
    reader: BitReader<'a>,
}

impl<'a> PesExtensionReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            reader: BitReader::new(data),
        }
    }

    pub fn read_flags(&self) -> Option<(bool, bool, bool, bool, bool)> {
        Some((
            self.reader.get_bit(0, 7)?,
            self.reader.get_bit(0, 6)?,
            self.reader.get_bit(0, 5)?,
            self.reader.get_bit(0, 4)?,
            self.reader.get_bit(0, 0)?,
        ))
    }

    pub fn read_private_data(&self, offset: usize) -> Option<u128> {
        let bytes = self.reader.get_bytes(offset, 16)?;
        Some(u128::from_be_bytes(bytes.try_into().ok()?))
    }

    pub fn read_sequence_counter(&self, offset: usize) -> Option<(u8, u8, u8)> {
        if !self.reader.get_bit(offset, 7)? {
            return None;
        }
        let counter = self.reader.get_bits(offset, 0x7F, 0)?;

        if !self.reader.get_bit(offset + 1, 7)? {
            return None;
        }
        let identifier = self.reader.get_bits(offset + 1, 0x40, 6)?;
        let stuff_length = self.reader.get_bits(offset + 1, 0x3F, 0)?;

        Some((counter, identifier, stuff_length))
    }

    pub fn read_buffer_info(&self, offset: usize) -> Option<(u8, u16)> {
        let scale = self.reader.get_bits(offset, 0x20, 5)?;
        let size = self.reader.get_bits_u16(offset, 0x1F, 0xFF)?;
        Some((scale, size))
    }
}
