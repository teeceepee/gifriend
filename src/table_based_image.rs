use crate::color_table::ColorTable;
use crate::image_data::ImageData;
use crate::image_descriptor::ImageDescriptor;

// separator: 44 (0x2C)
#[derive(Debug)]
pub struct TableBasedImage {
    image_descriptor: ImageDescriptor,
    local_color_table: ColorTable,
    pub image_data: ImageData,
}

impl TableBasedImage {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let image_descriptor = ImageDescriptor::parse_from_reader(rdr)?;

        let color_count = if image_descriptor.local_color_table_flag() == 1 {
            image_descriptor.color_count()
        } else {
            0
        };

        let local_color_table = ColorTable::parse_from_reader(rdr, color_count)?;

        let image_data = ImageData::parse_from_reader(rdr)?;

        let image = Self {
            image_descriptor,
            local_color_table,
            image_data,
        };

        Ok(image)
    }
}
