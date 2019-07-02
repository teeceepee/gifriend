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

    pub fn disposal_method(&self) -> u8 {
        (self.packed_fields & 0b0001_1100) >> 2
    }

    pub fn transparent_color_flag(&self) -> u8 {
        self.packed_fields & 0b0000_0001
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disposal_method() {
        let ext1 = GraphicControlExtension {
            block_size: 4,
            packed_fields: 0,
            delay_time: 1,
            transparent_color_index: 0,
            block_terminator: 0,
        };
        assert_eq!(0, ext1.disposal_method());

        let ext2 = GraphicControlExtension {
            block_size: 4,
            packed_fields: 5, // 0b000_001_10
            delay_time: 1,
            transparent_color_index: 0,
            block_terminator: 0,
        };
        assert_eq!(1, ext2.disposal_method());

        let ext3 = GraphicControlExtension {
            block_size: 4,
            packed_fields: 128, // 0b100_000_00
            delay_time: 1,
            transparent_color_index: 0,
            block_terminator: 0,
        };

        assert_eq!(0, ext3.disposal_method());
    }

    #[test]
    fn test_transparent_color_flag() {
        let ext1 = GraphicControlExtension {
            block_size: 4,
            packed_fields: 1,
            delay_time: 1,
            transparent_color_index: 0,
            block_terminator: 0,
        };
        assert_eq!(1, ext1.transparent_color_flag());

        let ext2 = GraphicControlExtension {
            block_size: 4,
            packed_fields: 4,
            delay_time: 1,
            transparent_color_index: 0,
            block_terminator: 0,
        };
        assert_eq!(0, ext2.transparent_color_flag());
    }
}
