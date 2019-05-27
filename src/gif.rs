use crate::header::Header;
use crate::logical_screen_descriptor::LogicalScreenDescriptor;
use crate::color_table::ColorTable;
use crate::data_items::{self, DataItems};

// https://www.w3.org/Graphics/GIF/spec-gif89a.txt

pub struct Gif {
    pub header: Header,
    pub logical_screen_descriptor: LogicalScreenDescriptor,
    pub global_color_table: Option<ColorTable>,

    pub data_items: DataItems,
}

impl Gif {
    pub fn parse(bytes: &[u8]) -> std::io::Result<Self> {
        let mut reader = std::io::Cursor::new(bytes);

        let header = Header::parse_from_reader(&mut reader)?;
        let logical_screen_descriptor = LogicalScreenDescriptor::parse_from_reader(&mut reader)?;

        let global_color_table = if logical_screen_descriptor.global_color_table_flag() == 1 {
            let color_table = ColorTable::parse_from_reader(&mut reader, logical_screen_descriptor.color_count())?;

            Some(color_table)
        } else {
            None
        };


        let data_items = data_items::parse_from_reader(&mut reader)?;

        let gif = Self {
            header,
            logical_screen_descriptor,
            global_color_table,
            data_items,
        };

        Ok(gif)
    }
}
