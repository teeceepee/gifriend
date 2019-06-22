use crate::header::Header;
use crate::logical_screen_descriptor::LogicalScreenDescriptor;
use crate::color_table::ColorTable;
use crate::data_item::DataItem;
use crate::extension::Extension;
use crate::extensions::application_extension::ApplicationExtension;
use crate::table_based_image::TableBasedImage;

// https://www.w3.org/Graphics/GIF/spec-gif89a.txt

pub struct Gif {
    pub header: Header,
    pub logical_screen_descriptor: LogicalScreenDescriptor,
    pub global_color_table: Option<ColorTable>,

    pub data_items: Vec<DataItem>,

    application_indices: Vec<usize>,
    comment_indices: Vec<usize>,
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

        let mut data_items: Vec<DataItem>= Vec::new();

        let mut application_indices = Vec::new();
        let mut comment_indices = Vec::new();

        let mut i = 0;
        loop {
            let item = DataItem::parse_from_reader(&mut reader)?;
            let mut is_trailer = false;

            match &item {
                DataItem::Extension(ext) => {
                    match ext {
                        Extension::Application(_app_ext) => {
                            application_indices.push(i);
                        },
                        Extension::Comment(_comment_ext) => {
                            comment_indices.push(i);
                        },
                        Extension::Control(_) => {
                            // graphic control
                        },
                        Extension::Text(_) => {
                            // rendering
                        }
                    }
                },
                DataItem::Image(_img) => {
                    // rendering
                },
                DataItem::Trailer => {
                    is_trailer = true;
                }
            }

            data_items.push(item);

            if is_trailer {
                break;
            }

            i += 1;
        }


        let gif = Self {
            header,
            logical_screen_descriptor,
            global_color_table,
            data_items,
            application_indices,
            comment_indices,
        };

        Ok(gif)
    }

    pub fn width(&self) -> u16 {
        self.logical_screen_descriptor.logical_screen_width
    }

    pub fn height(&self) -> u16 {
        self.logical_screen_descriptor.logical_screen_height
    }

    pub fn images(&self) -> Vec<&TableBasedImage> {
        let mut result = Vec::new();

        for item in self.data_items.iter() {
            match item {
                DataItem::Image(img) => {
                    result.push(img);
                },
                _ => {}
            }
        }

        result
    }

    pub fn applications(&self) -> Vec<String> {
        let mut result = Vec::new();

        for i in self.application_indices.iter() {
            let item = &self.data_items[*i];

            match item {
                DataItem::Extension(ext) => {
                    match ext {
                        Extension::Application(app_ext) => {
                            result.push(app_ext.to_s());
                        },
                        _ => {
                            continue;
                        }
                    }
                },
                _ => {
                    continue;
                }
            }
        }

        result
    }

    pub fn comments(&self) -> Vec<String> {
        let mut result = Vec::new();

        for i in self.comment_indices.iter() {
            let item = &self.data_items[*i];

            match item {
                DataItem::Extension(ext) => {
                    match ext {
                        Extension::Comment(comment_ext) => {
                            result.push(comment_ext.to_s());
                        },
                        _ => {
                            continue;
                        }
                    }
                },
                _ => {
                    continue;
                }
            }
        }

        result
    }
}
