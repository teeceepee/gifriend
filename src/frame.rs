use crate::color::Color;
use crate::color_table::ColorTable;
use crate::extensions::graphic_control_extension::GraphicControlExtension;
use crate::frame_utils::{select_colors, cal_screen_offset};
use crate::table_based_image::TableBasedImage;

///// Disposal Method
///// Indicates the way in which the graphic is to be treated after being displayed.
//#[repr(u8)]
//pub enum DisposalMethod {
//    /// No disposal specified. The is not required to take any action.
//    NoSpecified = 0,
//    /// Do not dispose. The graphic is to be left in place.
//    NotDispose = 1,
//    /// Restore to background color. The area used by the graphic must be restored to the
//    /// background color.
//    Background = 2,
//    /// Restore to previous. The decoder is required to restore the area overwritten by the graphic
//    /// with what was there prior to rendering the graphic.
//    Previous = 3,
//    // 4~7 To be defined.
//}
//
//impl From<u8> for DisposalMethod {
//    fn from(m: u8) -> DisposalMethod {
//        match m {
//            0 => DisposalMethod::NoSpecified,
//            1 => DisposalMethod::NotDispose,
//            2 => DisposalMethod::Background,
//            3 => DisposalMethod::Previous,
//
//            _ => DisposalMethod::NoSpecified,
//        }
//    }
//}

/// 解析后的每一帧，`delay_time` 的单位是 10ms，`bytes` 保存的是 RGBA 格式（每像素四字节）的图像数据
pub struct Frame {
    pub delay_time: u16,
    pub disposal_method: u8,
    //pub interlaced: bool,
    pub bytes: Vec<u8>,
}

impl Frame {
    // 根据 graphic control extension 和 table based image 解析出一帧
    pub fn new(
        screen_width: u16,
        screen_height: u16,
        global_color_table: &ColorTable,
        ctrl: Option<GraphicControlExtension>,
        img: &TableBasedImage,
        prev_frame: Option<&Frame>,
    ) -> Self {
        // graphic control extension 是可选的，
        // 如果存在，从中取出延迟时间和透明色下标；
        // 如果不存在，分别取 0 和 None
        let result = if let Some(control) = ctrl {
            let tc_index = if control.transparent_color_flag() == 1 {
                Some(control.transparent_color_index)
            } else {
                None
            };

            let method = control.disposal_method();

            (control.delay_time, tc_index, method)
        } else {
            (0, None, 0)
        };

        let (delay_time, transparent_index, disposal_method) = result;

        // 选取颜色
        let colors = select_colors(global_color_table, img);
        // 提取帧数据
        let bytes = Self::extract_frame_bytes(
            screen_width,
            screen_height,
            colors,
            transparent_index,
            img,
            prev_frame,
        );

        Self {
            delay_time,
            disposal_method,
            bytes,
        }
    }

    fn extract_frame_bytes(
        screen_width: u16,
        screen_height: u16,
        colors: &Vec<Color>,
        transparent_index: Option<u8>,
        img: &TableBasedImage,
        prev_frame: Option<&Frame>,
    ) -> Vec<u8> {
        let descriptor = &img.image_descriptor;
        let d = &img.decompressed;

        let mut frame_bytes: Vec<u8> = if let Some(pf) = prev_frame {
            // 如果前一帧存在，初始化为前一帧
            pf.bytes.to_owned()
        } else {
            // 如果 prev_frame 不存在，说明当前是第一帧，初始化为全透明
            let byte_count = 4 * screen_width as usize * screen_height as usize;
            let mut first_frame_bytes: Vec<u8> = Vec::with_capacity(byte_count);

            // 初始化为全透明像素
            for _ in 0..byte_count {
                first_frame_bytes.push(0);
            }

            first_frame_bytes
        };

        let slice = frame_bytes.as_mut_slice();

        for (n, &i) in d.iter().enumerate() {
            let mut index = i as usize;

            index = if let Some(trans_index) = transparent_index {
                // 透明
                if index == (trans_index as usize)  {
                    colors.len()
                } else {
                    index
                }
            } else {
                index
            };


            if index >= colors.len() {
                // 下标超出颜色表范围
            } else {
                let offset = 4 * cal_screen_offset(n, screen_width, screen_height, descriptor);

                if offset >= slice.len() {
                    // 偏移量超出帧数组长度，说明图片的这部分应该被截掉
                } else {
                    let color = &colors[index];

                    slice[offset] = color.red;
                    slice[offset + 1] = color.green;
                    slice[offset + 2] = color.blue;
                    slice[offset + 3] = 255;
                }
            }
        }

        frame_bytes
    }

}

impl Default for Frame {
    fn default() -> Frame {
        Frame {
            delay_time: 0,
            disposal_method: 0,
            bytes: Vec::new(),
        }
    }
}
