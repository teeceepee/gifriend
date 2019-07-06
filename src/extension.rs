use byteorder::ReadBytesExt;

use crate::extensions::application_extension::ApplicationExtension;
use crate::extensions::comment_extension::CommentExtension;
use crate::extensions::graphic_control_extension::GraphicControlExtension;
use crate::extensions::plain_text_extension::PlainTextExtension;

#[derive(Debug)]
pub enum Extension {
    Application(ApplicationExtension), // label: 255 (0xFF)
    Comment(CommentExtension), // label: 254 (0xFE)
    Control(GraphicControlExtension), // label: 249 (0xF9)
    Text(PlainTextExtension), // label: 1 (0x01)
}

impl Extension {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let label = rdr.read_u8()?;

        let v = match label {
            1 => {
                Extension::Text(PlainTextExtension::parse_from_reader(rdr)?)
            }
            249 => {
                Extension::Control(GraphicControlExtension::parse_from_reader(rdr)?)
            },
            254 => {
                Extension::Comment(CommentExtension::parse_from_reader(rdr)?)
            },
            _ => { // 255
                Extension::Application(ApplicationExtension::parse_from_reader(rdr)?)
            },
        };

        Ok(v)
    }
}
