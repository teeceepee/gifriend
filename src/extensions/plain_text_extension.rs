use byteorder::{ReadBytesExt, LE};
use crate::data_sub_blocks::DataSubBlocks;

// separator: 33, label:
#[derive(Debug)]
pub struct PlainTextExtension {
    block_size: u8,

    text_grid_left_position: u16,
    text_grid_top_position: u16,
    text_grid_width: u16,
    text_grid_height: u16,

    character_cell_width: u8,
    character_cell_height: u8,

    text_foreground_color_index: u8,
    text_background_color_index: u8,

    plain_text_data_and_terminator: DataSubBlocks,
}

impl PlainTextExtension {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let block_size = rdr.read_u8()?;

        let text_grid_left_position = rdr.read_u16::<LE>()?;
        let text_grid_top_position = rdr.read_u16::<LE>()?;
        let text_grid_width = rdr.read_u16::<LE>()?;
        let text_grid_height = rdr.read_u16::<LE>()?;

        let character_cell_width = rdr.read_u8()?;
        let character_cell_height = rdr.read_u8()?;

        let text_foreground_color_index = rdr.read_u8()?;
        let text_background_color_index = rdr.read_u8()?;

        let plain_text_data_and_terminator = DataSubBlocks::parse_from_reader(rdr)?;

        let ext = Self {
            block_size,
            text_grid_left_position,
            text_grid_top_position,
            text_grid_width,
            text_grid_height,
            character_cell_width,
            character_cell_height,
            text_foreground_color_index,
            text_background_color_index,
            plain_text_data_and_terminator,
        };

        Ok(ext)
    }
}
