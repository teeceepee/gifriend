use byteorder::{ReadBytesExt, LE};

#[derive(Debug)]
pub struct ImageDescriptor {
    left_position: u16,
    top_position: u16,
    width: u16,
    height: u16,

    // packed fields:
    // bit1 :local_color_table_flag
    // bit1 :interlace_flag
    // bit1 :sort_flag
    // bit2 :reserved
    // bit3 :size_of_local_color_table
    packed_fields: u8,
}

impl ImageDescriptor {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let left_position = rdr.read_u16::<LE>()?;
        let top_position = rdr.read_u16::<LE>()?;
        let width = rdr.read_u16::<LE>()?;
        let height = rdr.read_u16::<LE>()?;

        let packed_fields = rdr.read_u8()?;

        let desc = Self {
            left_position,
            top_position,
            width,
            height,
            packed_fields,
        };

        Ok(desc)
    }

    pub fn color_count(&self) -> u32 {
        let size_of_local_color_table = self.packed_fields & 0b0000_0111;
        2u32.pow(size_of_local_color_table as u32 + 1)
    }

    pub fn local_color_table_flag(&self) -> u8 {
        self.packed_fields >> 7
    }
}
