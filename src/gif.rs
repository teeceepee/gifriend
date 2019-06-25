use crate::header::Header;
use crate::logical_screen_descriptor::LogicalScreenDescriptor;
use crate::color_table::ColorTable;
use crate::data_item::DataItem;
use crate::extension::Extension;
use crate::frame::Frame;
use crate::extensions::graphic_control_extension::GraphicControlExtension;

// https://www.w3.org/Graphics/GIF/spec-gif89a.txt

pub struct Gif {
    _header: Header,
    logical_screen_descriptor: LogicalScreenDescriptor,
    _global_color_table: ColorTable,

    data_items: Vec<DataItem>,

    application_indices: Vec<usize>,
    comment_indices: Vec<usize>,

    pub frames: Vec<Frame>,
}

impl Gif {
    pub fn parse(bytes: &[u8]) -> std::io::Result<Self> {
        let mut reader = std::io::Cursor::new(bytes);

        let header = Header::parse_from_reader(&mut reader)?;
        let logical_screen_descriptor: LogicalScreenDescriptor = LogicalScreenDescriptor::parse_from_reader(&mut reader)?;

        let global_color_table: ColorTable = if logical_screen_descriptor.global_color_table_flag() == 1 {
            let color_table = ColorTable::parse_from_reader(&mut reader, logical_screen_descriptor.color_count())?;

            color_table
        } else {
            ColorTable::default()
        };

        let mut data_items: Vec<DataItem>= Vec::new();

        let mut application_indices = Vec::new();
        let mut comment_indices = Vec::new();

        let mut last_ctrl: Option<GraphicControlExtension> = None;
        let mut frames: Vec<Frame> = Vec::new();

        let mut i = 0;
        loop {
            let item: DataItem = DataItem::parse_from_reader(&mut reader)?;
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
                        Extension::Control(ctrl) => {
                            last_ctrl = Some(*ctrl);
                        },
                        Extension::Text(_) => {
                            // rendering
                        }
                    }
                },
                DataItem::Image(img) => {
                    // rendering
                    let frame = Frame::new(
                        logical_screen_descriptor.logical_screen_width,
                        logical_screen_descriptor.logical_screen_height,
                        &global_color_table,
                        last_ctrl,
                        &img
                    );

                    frames.push(frame);
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
            _header: header,
            logical_screen_descriptor,
            _global_color_table: global_color_table,
            data_items,
            application_indices,
            comment_indices,

            frames,
        };

        Ok(gif)
    }

    pub fn width(&self) -> u16 {
        self.logical_screen_descriptor.logical_screen_width
    }

    pub fn height(&self) -> u16 {
        self.logical_screen_descriptor.logical_screen_height
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
