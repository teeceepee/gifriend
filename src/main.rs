use std::io::Read;
//use gifriend::data_item::DataItem;

// https://www.w3.org/Graphics/GIF/spec-gif89a.txt

fn read() -> std::io::Result<()> {
//    let path = "samples/sample.gif";
//    let path = "samples/test.gif";
    let path = "samples/kobe.gif";
//    let path = "samples/s.gif";

    let mut file = std::fs::File::open(path)?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    if let Ok(f) = gifriend::parse(&bytes) {
        let frames = &f.frames;

        println!("gif: {} {}, frame_count: {}", f.width(), f.height(), frames.len());

        let f = &frames[0];

        println!("{:?}", f.bytes);
    }


    Ok(())
}


fn main() {
    read().unwrap();
}
