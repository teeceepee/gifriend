mod gif;

mod header;
mod logical_screen_descriptor;
mod color;
mod color_table;
mod data_sub_block;
mod data_sub_blocks;

mod extensions;
mod extension;

mod image_data;
mod image_descriptor;
mod table_based_image;

mod data_item;

mod frame;
mod frame_utils;
mod utils;

// https://www.w3.org/Graphics/GIF/spec-gif89a.txt

pub use frame::Frame;
pub use gif::Gif;

pub fn parse(gif_bytes: &[u8]) -> std::io::Result<Gif> {
    Gif::parse(gif_bytes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse1() {
        // 2x2 GIF（原文件的路径 test_samples/2x2.gif）
        let gif_bytes = [
            71, 73, 70, 56, 57, 97,        // header
            2, 0, 2, 0, 128, 0, 0,         // logical screen descriptor
            255, 255, 255, 0, 0, 0,        // global color table
            33, 249, 4, 4, 0, 0, 0, 0,     // graph control extension

            44, 0, 0, 0, 0, 2, 0, 2, 0, 0, // image descriptor
            2,                             // lzw min code size
            3, 12, 16, 5, 0,               // data blocks [12, 16, 5]
            59,                            // trailer
        ];

        if let Ok(gif) = parse(&gif_bytes) {
            assert_eq!(2, gif.width());
            assert_eq!(2, gif.height());
            assert_eq!(1, gif.frames.len());

            let expected_frame: [u8; 16] = [
                0, 0, 0, 255,
                255, 255, 255, 255,
                255, 255, 255, 255,
                0, 0, 0, 255,
            ];

            let f = gif.frames.first().unwrap();
            assert_eq!(expected_frame, f.bytes.as_slice());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse2() -> std::io::Result<()> {
        let path = "test_samples/beacon.gif";

        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::new();
        use std::io::Read;
        file.read_to_end(&mut bytes)?;

        if let Ok(gif) = parse(&bytes) {
            assert_eq!(6, gif.width());
            assert_eq!(6, gif.height());
            assert_eq!(2, gif.frames.len());
        } else {
            panic!();
        }

        Ok(())
    }
}
