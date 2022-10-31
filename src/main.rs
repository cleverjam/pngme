use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use std::str::FromStr;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let chunk = Chunk::new(ChunkType::from_str("RUST")?, vec![123]);

    println!("chunk display: {}", chunk);
    Ok(())
}
