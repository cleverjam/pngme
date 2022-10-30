mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::str::FromStr;
use crate::chunk_type::ChunkType;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let chunk = ChunkType::from_str("ruSt").unwrap();

    println!("is_error: {}", chunk.is_critical());
    Ok(())
}
