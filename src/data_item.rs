use byteorder::ReadBytesExt;
use crate::extension::Extension;
use crate::table_based_image::TableBasedImage;

#[derive(Debug)]
pub enum DataItem {
    Extension(Extension), // separator: 33
    Image(TableBasedImage), // separator: 44
    Trailer, // separator: 59
}

impl DataItem {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let separator = rdr.read_u8()?;

        let result = match separator {
            33 => {
                Ok(DataItem::Extension(Extension::parse_from_reader(rdr)?))
            },
            44 => {
                Ok(DataItem::Image(TableBasedImage::parse_from_reader(rdr)?))
            },
            59 => {
                Ok(DataItem::Trailer)
            },
            _ => {
                let msg = format!("Unknown separator: `{}`", separator);

                Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
            }
        };

        result
    }

    pub fn is_trailer(&self) -> bool {
        match self {
            DataItem::Trailer => true,
            _ => false
        }
    }
}
