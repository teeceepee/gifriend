use crate::data_sub_block::DataSubBlock;

#[derive(Debug)]
pub struct DataSubBlocks {
    pub sub_blocks: Vec<DataSubBlock>,
}

impl DataSubBlocks {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let mut sub_blocks: Vec<DataSubBlock> = Vec::new();

        loop {
            let sub_block = DataSubBlock::parse_from_reader(rdr)?;
            let block_size = sub_block.block_size;

            sub_blocks.push(sub_block);

            if block_size == 0 {
                break;
            }
        }

        let blocks = Self {
            sub_blocks,
        };

        Ok(blocks)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        for blk in self.sub_blocks.iter() {
            for b in blk.data_values.iter() {
                result.push(*b);
            }
        }

        result
    }
}
