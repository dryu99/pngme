use std::fmt;

use crate::chunk_type::ChunkType;
use crate::{Error, Result};

pub struct Chunk<'a> {
    length: u32,
    r#type: ChunkType,
    data: &'a [u8],
    crc: u32, // TODO maybe [u8; 4]
}

impl TryFrom<&[u8]> for Chunk<'_> {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        todo!()
    }
}

impl fmt::Display for Chunk<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
