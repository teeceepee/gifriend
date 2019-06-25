//use crate::extensions::graphic_control_extension::GraphicControlExtension;
//use crate::graphic_rendering_block::GraphicRenderingBlock;
//
//// Deprecated
//// separator: 33, label: 249
//#[derive(Debug)]
//pub struct GraphicBlock {
//    graphic_control_extension: GraphicControlExtension,
//    graphic_rendering_block: GraphicRenderingBlock,
//}
//
//impl GraphicBlock {
//    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
//        let graphic_control_extension = GraphicControlExtension::parse_from_reader(rdr)?;
//        let graphic_rendering_block =  GraphicRenderingBlock::parse_from_reader(rdr)?;
//
//        let block = Self {
//            graphic_control_extension,
//            graphic_rendering_block,
//        };
//
//        Ok(block)
//    }
//}
