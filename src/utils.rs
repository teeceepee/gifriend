use lzw::{Decoder, LsbReader};

// TODO remove unwrap
pub fn lzw_decode(compressed_data: &[u8], min_code_size: u8) -> Vec<u8> {
    let mut decoded: Vec<u8> = Vec::new();

    let mut decoder = Decoder::new(LsbReader::new(), min_code_size);
    let mut compressed = &compressed_data[..];

    while compressed.len() > 0 {
        let (start, bytes) = decoder.decode_bytes(&compressed).unwrap();

        compressed = &compressed[start..];
        decoded.extend(bytes.iter().map(|&i| i));
    }

    decoded
}
