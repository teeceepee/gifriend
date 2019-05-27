use crate::data_item::DataItem;

pub type DataItems = Vec<DataItem>;

pub fn parse_from_reader(rdr: &mut std::io::Cursor<&[u8]>) -> std::io::Result<DataItems> {
    let mut items: DataItems = Vec::new();

    loop {
        let item = DataItem::parse_from_reader(rdr)?;
        let is_trailer = item.is_trailer();

        items.push(item);

        if is_trailer {
            break;
        }
    }

    Ok(items)
}
