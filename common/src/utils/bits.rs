#[derive(Debug)]
pub struct BitReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    pub fn at_position(data: &'a [u8], position: usize) -> Self {
        Self { data, position }
    }

    pub fn get_bit(&self, byte_offset: usize, position: u8) -> Option<bool> {
        self.data
            .get(self.position + byte_offset)
            .map(|byte| (*byte & (1 << position)) != 0)
    }

    pub fn get_bits(&self, byte_offset: usize, mask: u8, shift: u8) -> Option<u8> {
        self.data
            .get(self.position + byte_offset)
            .map(|byte| (*byte & mask) >> shift)
    }

    pub fn get_bits_u16(&self, byte_offset: usize, upper_mask: u8, lower_mask: u8) -> Option<u16> {
        if self.position + byte_offset + 1 >= self.data.len() {
            return None;
        }

        let upper = (self.data[self.position + byte_offset] & upper_mask) as u16;
        let lower = (self.data[self.position + byte_offset + 1] & lower_mask) as u16;

        Some((upper << 8) | lower)
    }

    pub fn get_bits_u24(&self, byte_offset: usize) -> Option<u32> {
        if self.position + byte_offset + 2 >= self.data.len() {
            return None;
        }

        let b0 = self.data[self.position + byte_offset] as u32;
        let b1 = self.data[self.position + byte_offset + 1] as u32;
        let b2 = self.data[self.position + byte_offset + 2] as u32;

        Some((b0 << 16) | (b1 << 8) | b2)
    }

    pub fn get_bytes(&self, byte_offset: usize, length: usize) -> Option<Vec<u8>> {
        if self.position + byte_offset + length > self.data.len() {
            return None;
        }
        Some(self.data[self.position + byte_offset..self.position + byte_offset + length].to_vec())
    }

    pub fn remaining_from(&self, byte_offset: usize) -> Option<Vec<u8>> {
        if self.position + byte_offset >= self.data.len() {
            return None;
        }
        Some(self.data[self.position + byte_offset..].to_vec())
    }

    pub fn advance(&mut self, bytes: usize) {
        self.position += bytes;
    }

    pub fn current_position(&self) -> usize {
        self.position
    }

    pub fn get_pid(&self, byte_offset: usize, upper_mask: u8) -> Option<u16> {
        self.get_bits_u16(byte_offset, upper_mask, 0xFF)
    }

    pub fn read_program_entry(&self, byte_offset: usize, upper_mask: u8) -> Option<(u16, u16)> {
        let program_number = self.get_bits_u16(byte_offset, 0xFF, 0xFF)?;
        let pid = self.get_pid(byte_offset + 2, upper_mask)?;
        Some((program_number, pid))
    }

    pub fn read_es_info(&self, byte_offset: usize, upper_masks: (u8, u8)) -> Option<(u16, u16)> {
        let elementary_pid = self.get_bits_u16(byte_offset, upper_masks.0, 0xFF)?;
        let es_info_length = self.get_bits_u16(byte_offset + 2, upper_masks.1, 0xFF)?;
        Some((elementary_pid, es_info_length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::BitManipulation;

    #[test]
    fn test_bit_reader() {
        let data = [0b10100110, 0b11110000];
        let reader = BitReader::new(&data);

        assert_eq!(reader.get_bit(0, 7), Some(true));
        assert_eq!(reader.get_bit(0, 6), Some(false));
        assert_eq!(reader.get_bits(0, 0b11100000, 5), Some(0b101));
        assert_eq!(
            reader.get_bits_u16(0, 0b11111111, 0b11111111),
            Some(0b1010011011110000)
        );
    }

    #[test]
    fn test_bit_manipulation() {
        struct TestStruct;
        impl BitManipulation for TestStruct {}

        assert!(TestStruct::get_bit(0b10000000, 7));
        assert!(!TestStruct::get_bit(0b01111111, 7));
        assert_eq!(TestStruct::get_bits(0b11000000, 0b11000000, 6), 0b11);
    }

    #[test]
    fn test_get_bits_u24() {
        let data = [0x12, 0x34, 0x56, 0x78];
        let reader = BitReader::new(&data);
        assert_eq!(reader.get_bits_u24(0), Some(0x123456));
    }

    #[test]
    fn test_read_es_info() {
        let data = [0x1F, 0xFF, 0x0F, 0xFF];
        let reader = BitReader::new(&data);
        assert_eq!(reader.read_es_info(0, (0x1F, 0x0F)), Some((0x1FFF, 0x0FFF)));
    }
}
