use std::io::Read;
//use gifriend::data_item::DataItem;

// https://www.w3.org/Graphics/GIF/spec-gif89a.txt

fn read() -> std::io::Result<()> {
//    let path = "samples/sample.gif";
//    let path = "samples/test.gif";
    let path = "samples/kobe.gif";

    let mut file = std::fs::File::open(path)?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;


    let g: gifriend::gif::Gif = gifriend::gif::Gif::parse(&bytes)?;


//    for i in g.data_items.iter() {
//
//        match i {
//            DataItem::Image(_) => println!("Image"),
//            _ => println!("{:?}", i),
//        }
//    }

    println!("w: {}, h: {}", g.width(), g.height());
    println!("data_items size: {:#?}", g.data_items.len());

    println!("applications: {:?}", g.applications());
    println!("comments: {:?}", g.comments());


    let img = g.images()[0];
    println!("image data: {:?}", img.image_data);


    Ok(())
}


fn main() {
    read().unwrap();
}
