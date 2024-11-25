pub struct ByteOperations;

impl ByteOperations {
    pub fn find_pattern(data: &[u8], pattern: &[u8]) -> Option<usize> {
        data.windows(pattern.len())
            .position(|window| window == pattern)
    }

    pub fn find_padding_end(data: &[u8], padding_byte: u8, consecutive: usize) -> Option<usize> {
        let mut count = 0;
        for (i, &byte) in data.iter().enumerate() {
            if byte == padding_byte {
                count += 1;
                if count == consecutive {
                    return Some(i - (consecutive - 1));
                }
            } else {
                count = 0;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pattern() {
        let data = &[1, 2, 3, 4, 5];
        assert_eq!(ByteOperations::find_pattern(data, &[2, 3]), Some(1));
        assert_eq!(ByteOperations::find_pattern(data, &[5, 6]), None);
    }

    #[test]
    fn test_find_padding_end() {
        let data = &[1, 0xFF, 0xFF, 0xFF, 2];
        assert_eq!(ByteOperations::find_padding_end(data, 0xFF, 3), Some(1));
    }
}
