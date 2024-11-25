pub struct Crc32Reader {
    data: Vec<u8>,
}

impl Crc32Reader {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn read_crc32(&self) -> Option<u32> {
        if self.data.len() < 4 {
            return None;
        }
        let last_four = &self.data[self.data.len() - 4..];
        Some(u32::from_be_bytes(last_four.try_into().ok()?))
    }

    pub fn data_without_crc(&self) -> &[u8] {
        &self.data[..self.data.len().saturating_sub(4)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_reader() {
        let data = vec![0x01, 0x02, 0x03, 0x12, 0x34, 0x56, 0x78];
        let reader = Crc32Reader::new(&data);
        assert_eq!(reader.read_crc32(), Some(0x12345678));
        assert_eq!(reader.data_without_crc(), &[0x01, 0x02, 0x03]);
    }
}
