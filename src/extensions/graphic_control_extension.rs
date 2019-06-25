use byteorder::{ReadBytesExt, LE};

// separator: 33, label: 249
#[derive(Debug, Copy, Clone)]
pub struct GraphicControlExtension {
    block_size: u8,

    // packed fields:
    // bit3 :reserved
    // bit3 :disposal_method
    // bit1 :user_input_flag
    // bit1 :transparent_color_flag
    packed_fields: u8,

    pub delay_time: u16,
    pub transparent_color_index: u8,
    block_terminator: u8,
}

impl GraphicControlExtension {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let block_size = rdr.read_u8()?;
        let packed_fields = rdr.read_u8()?;
        let delay_time = rdr.read_u16::<LE>()?;
        let transparent_color_index = rdr.read_u8()?;
        let block_terminator = rdr.read_u8()?;

        let ext = Self {
            block_size,
            packed_fields,
            delay_time,
            transparent_color_index,
            block_terminator,
        };

        Ok(ext)
    }

    pub fn transparent_color_flag(&self) -> u8 {
        self.packed_fields & 0b0000_0001
    }
}
