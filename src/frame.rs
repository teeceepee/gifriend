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

/// 解析后的每一帧，`delay_time` 的单位是 10ms，`bytes` 保存的是 RGBA 格式（每像素四字节）的图像数据
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
        img: &TableBasedImage,
    ) -> Self {
        // graphic control extension 是可选的，
        // 如果存在，从中取出延迟时间和透明色下标；如果不存在，分别取 0 和 None
        let (delay_time, transparent_index) = if let Some(control) = ctrl {
            let tc_index = if control.transparent_color_flag() == 1 {
                Some(control.transparent_color_index)
            } else {
                None
            };

            (control.delay_time, tc_index)
        } else {
            (0, None)
        };

        let bytes = img.convert_to_frame(
            screen_width,
            screen_height,
            color_table,
            transparent_index,
        );

        Self {
            delay_time,
            bytes,
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
