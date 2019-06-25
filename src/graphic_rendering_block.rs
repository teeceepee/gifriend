//use byteorder::ReadBytesExt;
//use crate::extensions::plain_text_extension::PlainTextExtension;
//use crate::table_based_image::TableBasedImage;
//
//// Deprecated
//#[derive(Debug)]
//pub enum GraphicRenderingBlock {
//    Image(TableBasedImage),
//    Text(PlainTextExtension),
//}
//
//impl GraphicRenderingBlock {
//    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
//        // 44 (0x2C) => table based image
//        // 33 (0x21) => plain text extension
//        let separator = rdr.read_u8()?;
//
//        match separator {
//            44 => {
//                let image = TableBasedImage::parse_from_reader(rdr)?;
//
//                Ok(GraphicRenderingBlock::Image(image))
//            },
//            33 => {
//                let text = PlainTextExtension::parse_from_reader(rdr)?;
//
//                Ok(GraphicRenderingBlock::Text(text))
//            }
//            _ => {
//                let msg = format!("Unknown separator: `{}`", separator);
//
//                Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
//            }
//        }
//    }
//}
