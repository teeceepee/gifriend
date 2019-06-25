use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let mut c = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        c.red = rdr.read_u8()?;
        c.green = rdr.read_u8()?;
        c.blue = rdr.read_u8()?;

        Ok(c)
    }
}
