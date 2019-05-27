use byteorder::ReadBytesExt;
use crate::data_sub_blocks::DataSubBlocks;

#[derive(Debug)]
pub struct ImageData {
    lzw_minimum_code_size: u8,
    image_data: DataSubBlocks,
}

impl ImageData {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let lzw_minimum_code_size = rdr.read_u8()?;
        let image_data = DataSubBlocks::parse_from_reader(rdr)?;

        let data = Self {
            lzw_minimum_code_size,
            image_data,
        };

        Ok(data)
    }
}
