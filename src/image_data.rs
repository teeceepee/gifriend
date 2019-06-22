use byteorder::ReadBytesExt;
use crate::data_sub_blocks::DataSubBlocks;
use crate::utils::lzw_decode;

#[derive(Debug)]
pub struct ImageData {
    lzw_minimum_code_size: u8,
    pub compressed_data: Vec<u8>,
    //image_data: DataSubBlocks,
}

impl ImageData {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let lzw_minimum_code_size = rdr.read_u8()?;
        let image_data: DataSubBlocks = DataSubBlocks::parse_from_reader(rdr)?;

        let compressed_data = image_data.to_bytes();

        let data = Self {
            lzw_minimum_code_size,
            compressed_data,
            //image_data,
        };

        Ok(data)
    }

    pub fn decompress(&self) -> Vec<u8> {
        lzw_decode(&self.compressed_data, self.lzw_minimum_code_size)
    }
}
