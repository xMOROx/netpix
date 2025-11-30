pub struct TimestampReader<'a> {
    reader: crate::utils::BitReader<'a>,
}

impl<'a> TimestampReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            reader: crate::utils::BitReader::new(data),
        }
    }

    pub fn read_timestamp(&self) -> Result<u64, ()> {
        if !self.validate_marker_bits(&[0, 2, 4], 0x01) {
            return Err(());
        }

        let ts_1 = ((self.reader.get_bits(0, 0x0E, 0).ok_or(())?) as u64) << 29;
        let ts_2 = (self.reader.get_bytes(1, 1).ok_or(())?[0] as u64) << 22;
        let ts_3 = ((self.reader.get_bits(2, 0xFE, 0).ok_or(())?) as u64) << 14;
        let ts_4 = (self.reader.get_bytes(3, 1).ok_or(())?[0] as u64) << 7;
        let ts_5 = (self.reader.get_bits(4, 0xFE, 1).ok_or(())?) as u64;

        Ok(ts_1 | ts_2 | ts_3 | ts_4 | ts_5)
    }

    pub fn read_tref(&self) -> Result<u64, ()> {
        if !self.validate_marker_bits(&[0, 2, 4], 0x01) {
            return Err(());
        }

        let tref_1 = ((self.reader.get_bits(0, 0x0E, 0).ok_or(())?) as u64) << 29;
        let tref_2 = (self.reader.get_bytes(1, 1).ok_or(())?[0] as u64) << 22;
        let tref_3 = ((self.reader.get_bits(2, 0xFE, 0).ok_or(())?) as u64) << 14;
        let tref_4 = (self.reader.get_bytes(3, 1).ok_or(())?[0] as u64) << 7;
        let tref_5 = (self.reader.get_bits(4, 0xFE, 1).ok_or(())?) as u64;

        Ok(tref_1 | tref_2 | tref_3 | tref_4 | tref_5)
    }

    pub fn read_escr(&self) -> Result<(u64, u16), ()> {
        let base_1 = ((self.reader.get_bits(0, 0b00111000, 0).ok_or(())?) as u64) << 27;
        if !self.validate_escr_marker_bits() {
            return Err(());
        }

        let base_2 = ((self.reader.get_bits(0, 0b00000011, 0).ok_or(())?) as u64) << 28;
        let base_3 = (self.reader.get_bytes(1, 1).ok_or(())?[0] as u64) << 20;
        let base_4 = ((self.reader.get_bits(2, 0b11111000, 0).ok_or(())?) as u64) << 12;
        let base_5 = ((self.reader.get_bits(2, 0b00000011, 0).ok_or(())?) as u64) << 13;
        let base_6 = (self.reader.get_bytes(3, 1).ok_or(())?[0] as u64) << 5;
        let base_7 = (self.reader.get_bits(4, 0b11111000, 3).ok_or(())?) as u64;

        let extension_1 = ((self.reader.get_bits(4, 0b00000011, 0).ok_or(())?) as u16) << 7;
        let extension_2 = (self.reader.get_bits(5, 0b11111110, 1).ok_or(())?) as u16;

        Ok((
            base_1 | base_2 | base_3 | base_4 | base_5 | base_6 | base_7,
            extension_1 | extension_2,
        ))
    }

    fn validate_marker_bits(&self, positions: &[usize], mask: u8) -> bool {
        positions
            .iter()
            .all(|&pos| self.reader.get_bits(pos, mask, 0) == Some(mask))
    }

    fn validate_escr_marker_bits(&self) -> bool {
        (self.reader.get_bits(0, 0b00000100, 2) == Some(1))
            && (self.reader.get_bits(2, 0b00000100, 2) == Some(1))
            && (self.reader.get_bits(4, 0b00000100, 2) == Some(1))
            && (self.reader.get_bits(5, 0b00000001, 0) == Some(1))
    }
}
