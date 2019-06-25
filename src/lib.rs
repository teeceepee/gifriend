mod gif;

mod header;
mod logical_screen_descriptor;
mod color;
mod color_table;
mod data_sub_block;
mod data_sub_blocks;

mod extensions;
mod extension;

mod image_data;
mod image_descriptor;
mod table_based_image;

mod graphic_block;
mod graphic_rendering_block;

mod data_item;

mod frame;
mod utils;

pub use frame::Frame;
pub use gif::Gif;

pub fn parse(gif_bytes: &[u8]) -> std::io::Result<Gif> {
    Gif::parse(gif_bytes)
}
