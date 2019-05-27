use std::io::Read;
use byteorder::ReadBytesExt;
use crate::data_sub_blocks::DataSubBlocks;

// separator: 33, label: 255
#[derive(Debug)]
pub struct ApplicationExtension {
    block_size: u8,
    application_identifier: [u8;8],
    application_authentication_code: [u8;3],
    application_data_and_block_terminator: DataSubBlocks,
}

impl ApplicationExtension {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let block_size = rdr.read_u8()?;

        let mut application_identifier = [0u8; 8];
        rdr.read_exact(&mut application_identifier)?;

        let mut application_authentication_code = [0u8; 3];
        rdr.read_exact(&mut application_authentication_code)?;

        let application_data_and_block_terminator = DataSubBlocks::parse_from_reader(rdr)?;

        let ext = Self {
            block_size,
            application_identifier,
            application_authentication_code,
            application_data_and_block_terminator,
        };

        Ok(ext)
    }
}
