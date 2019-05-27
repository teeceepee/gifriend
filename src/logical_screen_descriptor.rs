use byteorder::{ReadBytesExt, LE};

#[derive(Debug)]
pub struct LogicalScreenDescriptor {
    pub logical_screen_width: u16,
    pub logical_screen_height: u16,

    // packed fields:
    // bit1 :global_color_table_flag
    // bit3 :color_resolution
    // bit1 :sort_flag
    // bit3 :size_of_global_color_table
    packed_fields: u8,

    background_color_index: u8,
    pixel_aspect_ratio: u8,
}

impl LogicalScreenDescriptor {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let mut descriptor = Self {
            logical_screen_width: 0,
            logical_screen_height: 0,
            packed_fields: 0,
            background_color_index: 0,
            pixel_aspect_ratio: 0,
        };

        descriptor.logical_screen_width = rdr.read_u16::<LE>()?;
        descriptor.logical_screen_height = rdr.read_u16::<LE>()?;
        descriptor.packed_fields = rdr.read_u8()?;
        descriptor.background_color_index = rdr.read_u8()?;
        descriptor.pixel_aspect_ratio = rdr.read_u8()?;

        Ok(descriptor)
    }

    pub fn global_color_table_flag(&self) -> u8 {
        self.packed_fields >> 7
    }

    pub fn color_count(&self) -> u32 {
        let size_of_global_color_table = self.packed_fields & 0b0000_0111;
        2u32.pow(size_of_global_color_table as u32 + 1)
    }

    pub fn table_size(&self) -> u32 {
        3 * self.color_count() as u32
    }
}
