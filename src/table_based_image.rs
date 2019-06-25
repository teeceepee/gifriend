use crate::color_table::ColorTable;
use crate::image_data::ImageData;
use crate::image_descriptor::ImageDescriptor;

// separator: 44 (0x2C)
#[derive(Debug)]
pub struct TableBasedImage {
    image_descriptor: ImageDescriptor,
    local_color_table: ColorTable,
    pub decompressed: Vec<u8>,
    //image_data: ImageData,
}

impl TableBasedImage {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let image_descriptor: ImageDescriptor = ImageDescriptor::parse_from_reader(rdr)?;

        let color_count = if image_descriptor.local_color_table_flag() == 1 {
            image_descriptor.color_count()
        } else {
            0
        };

        let local_color_table = ColorTable::parse_from_reader(rdr, color_count)?;

        let image_data: ImageData = ImageData::parse_from_reader(rdr)?;
        let decompressed = image_data.decompress();

        let image = Self {
            image_descriptor,
            local_color_table,
            decompressed,
            //image_data,
        };

        Ok(image)
    }

    pub fn convert_to_frame(
        &self,
        screen_width: u16,
        screen_height: u16,
        global_color_table: &ColorTable,
        transparent_index: Option<u8>,
    ) -> Vec<u8> {
        let descriptor = &self.image_descriptor;

        let colors = if self.local_color_table.colors.len() > 0 {
            &self.local_color_table.colors
        } else {
            &global_color_table.colors
        };

        let d = &self.decompressed;

        let byte_count = 4 * screen_width as usize * screen_height as usize;
        let mut frame_bytes: Vec<u8> = Vec::with_capacity(byte_count);

        // 初始化为全透明像素
        for _ in 0..byte_count {
            frame_bytes.push(0);
        }

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

/// 计算在逻辑屏幕坐标的偏移量
///
/// 1. 图像偏移量转图像坐标
/// 2. 图像坐标转为逻辑屏幕坐标
/// 3. 逻辑屏幕坐标转逻辑屏幕偏移量
fn cal_screen_offset(n: usize, screen_width: u16, screen_height: u16, descriptor: &ImageDescriptor) -> usize {
    let left = descriptor.left_position;
    let top = descriptor.top_position;
    let w = descriptor.width;
    let h = descriptor.height;

    let mut c = offset_to_coord(n, w as usize, h as usize);

    c.0 += left as usize;
    c.1 += top as usize;

    coord_to_offset(c, screen_width as usize, screen_height as usize)
}

/// 偏移量转坐标
fn offset_to_coord(offset: usize, w: usize, _h: usize) -> (usize, usize) {
    let x = offset % w;
    let y = offset / w;

    (x, y)
}

/// 坐标转偏移量
fn coord_to_offset(cord: (usize, usize), w: usize, _h: usize) -> usize {
    cord.1 * w + cord.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_to_coord() {
        let (width, height) = (300, 200);

        assert_eq!((0, 0), offset_to_coord(0, width, height));
        assert_eq!((10, 0), offset_to_coord(10, width, height));
        assert_eq!((299, 0), offset_to_coord(299, width, height));
        assert_eq!((0, 1), offset_to_coord(300, width, height));
        assert_eq!((2, 1), offset_to_coord(302, width, height));
    }

    #[test]
    fn test_coord_to_offset() {
        let (width, height) = (500, 10);

        assert_eq!(0, coord_to_offset((0, 0), width, height));
        assert_eq!(1, coord_to_offset((1, 0), width, height));
        assert_eq!(499, coord_to_offset((499, 0), width, height));
        assert_eq!(500, coord_to_offset((0, 1), width, height));
        assert_eq!(503, coord_to_offset((3, 1), width, height));

    }
}
