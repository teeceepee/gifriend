use crate::data_sub_blocks::DataSubBlocks;

// separator: 33, label: 254
#[derive(Debug)]
pub struct CommentExtension {
    comment_data_and_block_terminator: DataSubBlocks,
}

impl CommentExtension {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let comment_data_and_block_terminator = DataSubBlocks::parse_from_reader(rdr)?;

        let ext = Self {
            comment_data_and_block_terminator,
        };

        Ok(ext)
    }

    pub fn to_s(&self) -> String {
        match String::from_utf8(self.comment_data_and_block_terminator.to_bytes()) {
            Ok(s) => s,
            Err(_) => String::from(""),
        }
    }
}
