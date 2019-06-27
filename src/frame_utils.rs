use crate::color::Color;
use crate::color_table::ColorTable;
use crate::image_descriptor::ImageDescriptor;
use crate::table_based_image::TableBasedImage;

/// 选择应该使用的颜色表颜色
/// 优先使用全局颜色表，不存在则使用 table based image 中的颜色表
pub fn select_colors<'a>(global_color_table: &'a ColorTable, img: &'a TableBasedImage) -> &'a Vec<Color> {
    if img.local_color_table.colors.len() > 0 {
        &img.local_color_table.colors
    } else {
        &global_color_table.colors
    }
}

/// 计算在逻辑屏幕坐标的偏移量
///
/// 1. 图像偏移量转图像坐标
/// 2. 图像坐标转为逻辑屏幕坐标
/// 3. 逻辑屏幕坐标转逻辑屏幕偏移量
pub fn cal_screen_offset(n: usize, screen_width: u16, screen_height: u16, descriptor: &ImageDescriptor) -> usize {
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
