use std::io::Read;

// i) Signature - Identifies the GIF Data Stream. This field contains
// the fixed value 'GIF'.
//
// ii) Version - Version number used to format the data stream.
// Identifies the minimum set of capabilities necessary to a decoder
// to fully process the contents of the Data Stream.
//
// Version Numbers as of 10 July 1990 :       "87a" - May 1987
//                                            "89a" - July 1989
pub struct Header {
    signature: [u8;3], // b"GIF"
    version: [u8;3], // b"89a" b"87a"
}

impl Header {
    pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        let mut h = Self {
            signature: [0; 3],
            version: [0; 3],
        };

        rdr.read_exact(&mut h.signature)?;
        rdr.read_exact(&mut h.version)?;

        Ok(h)
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sig_str = match std::str::from_utf8(&self.signature) {
            Ok(s) => s,
            Err(_) => "",
        };
        let ver_str =  match std::str::from_utf8(&self.version) {
            Ok(s) => s,
            Err(_) => "",
        };

        write!(f, "#<Header signature=\"{}\"  version=\"{}\" >", sig_str, ver_str)
    }
}
