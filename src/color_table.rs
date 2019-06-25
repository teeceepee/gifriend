use std::io::Cursor;
use crate::color::Color;

#[derive(Debug)]
pub struct ColorTable {
    pub colors: Vec<Color>,
}

impl ColorTable {
    pub fn parse_from_reader(mut rdr: &mut Cursor<&[u8]>, color_count: u32) -> std::io::Result<Self> {
        let mut colors: Vec<Color> = Vec::new();

        for _i in 0..color_count {
            let c = Color::parse_from_reader(&mut rdr)?;

            colors.push(c);
        }

        let ct = Self {
            colors,
        };

        Ok(ct)
    }
}

impl Default for ColorTable {
    fn default() -> Self {
        Self { colors: Vec::new() }
    }
}
