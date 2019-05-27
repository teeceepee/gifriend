use std::io::Read;
use gifriend::data_item::DataItem;

// https://www.w3.org/Graphics/GIF/spec-gif89a.txt

fn read() -> std::io::Result<()> {
    let path = "samples/sample.gif";
//    let path = "samples/test.gif";

    let mut file = std::fs::File::open(path)?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;


    let g = gifriend::gif::Gif::parse(&bytes)?;

    println!("data_items size: {:#?}", g.data_items.len());

    for i in g.data_items {

        match i {
            DataItem::Image(_) => println!("Image"),
            _ => println!("{:?}", i),
        }
    }

    Ok(())
}


fn main() {
    read().unwrap();
}
