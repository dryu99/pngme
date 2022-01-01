use crate::chunk_type::ChunkType;
use crate::{Error, Result};

pub struct Chunk<'a> {
    length: u32,
    r#type: ChunkType,
    data: &'a [u8],
    crc: u32, // TODO maybe [u8; 4]
}
