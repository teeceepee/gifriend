use lzw::{Decoder, LsbReader};

/// LZW 解压缩
pub fn lzw_decode(compressed_data: &[u8], min_code_size: u8) -> Vec<u8> {
    let mut decoded: Vec<u8> = Vec::new();

    let mut decoder = Decoder::new(LsbReader::new(), min_code_size);
    let mut compressed = &compressed_data[..];

    while compressed.len() > 0 {
        // TODO to remove unwrap
        let (start, bytes) = decoder.decode_bytes(&compressed).unwrap();

        compressed = &compressed[start..];
        decoded.extend(bytes.iter().map(|&i| i));
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let compressed = [12, 16, 5];
        let min_code_size = 2;

        let data = lzw_decode(&compressed, min_code_size);

        let expected: [u8; 4] = [1, 0, 0, 1];
        assert_eq!(expected, data.as_slice());
    }
}
