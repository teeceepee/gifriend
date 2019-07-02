fn main() {
    simple_example().unwrap();
}

fn simple_example() -> std::io::Result<()> {
    let path = "test_samples/beacon.gif";

    let mut file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    use std::io::Read;
    file.read_to_end(&mut bytes)?;

    let mut total = 0;

    if let Ok(gif) = gifriend::parse(&bytes) {
        for f in gif.frames.iter() {
            total += f.bytes.len();

            println!("method: {:?}", f.disposal_method);
        }

        println!("total: {}", total);
    } else {
        panic!();
    }

    Ok(())
}
