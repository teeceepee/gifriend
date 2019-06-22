use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct DataSubBlock {
    pub block_size: u8,
    pub data_values: Vec<u8>,
}

impl DataSubBlock {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let block_size = rdr.read_u8()?;
        let mut data_values: Vec<u8> = Vec::with_capacity(block_size as usize);

        if block_size > 0 {
            for _i in 0..block_size {
                data_values.push(rdr.read_u8()?);
            }
        }

        let b = Self {
            block_size,
            data_values,
        };

        Ok(b)
    }
}
