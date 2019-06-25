use crate::extensions::graphic_control_extension::GraphicControlExtension;
use crate::table_based_image::TableBasedImage;
use crate::color_table::ColorTable;

//#[repr(u8)]
//pub enum DisposalMethod {
//    /// StreamingDecoder is not required to take any action.
//    Any = 0,
//    /// Do not dispose.
//    Keep = 1,
//    /// Restore to background color.
//    Background = 2,
//    /// Restore to previous.
//    Previous = 3,
//}

pub struct Frame {
    pub delay_time: u16,
    //pub dispose_method: u8,
    //pub interlaced: bool,
    pub bytes: Vec<u8>,
}

impl Frame {
    pub fn new(
        screen_width: u16,
        screen_height: u16,
        color_table: &ColorTable,
        ctrl: Option<GraphicControlExtension>,
        img: &TableBasedImage
    ) -> Self {


        if let Some(control) = ctrl {
            let transparent_index = if control.transparent_color_flag() == 1 {
                Some(control.transparent_color_index)
            } else {
                None
            };

            let bytes = img.convert_to_frame(
                screen_width,
                screen_height,
                color_table,
                transparent_index,
            );

            Self {
                delay_time: control.delay_time,
                bytes,
            }
        } else {
            let mut bytes = img.convert_to_frame(
                screen_width,
                screen_height,
                color_table,
                None,
            );

            Self {
                delay_time: 0,
                bytes,
            }
        }
    }
}

impl Default for Frame {
    fn default() -> Frame {
        Frame {
            delay_time: 0,
            bytes: Vec::new(),
        }
    }
}
