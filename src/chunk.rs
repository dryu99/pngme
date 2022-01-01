use std::fmt;
use std::io::{BufReader, Read};

use crate::chunk_type::ChunkType;
use crate::{Error, Result};

pub struct Chunk {
    length: u32,
    r#type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    // TODO use ? operator for read_exact calls
    fn try_from(bytes: &[u8]) -> Result<Self> {
        // TODO validate len?
        let mut reader = BufReader::new(bytes);

        // read length
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer);
        let chunk_length = u32::from_be_bytes(buffer);
        println!("chunk length: {}", chunk_length);

        // read chunk type
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer);
        let chunk_type = ChunkType::try_from(buffer)?;
        println!("chunk type: {}", chunk_type);

        // read chunk data
        let mut chunk_data = vec![0u8; chunk_length as usize];
        reader.read_exact(&mut chunk_data);
        println!(
            "chunk data: {}",
            String::from_utf8(chunk_data.clone()).unwrap()
        );

        // read crc
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer);
        let crc = u32::from_be_bytes(buffer);
        println!("crc: {}", crc);

        Ok(Self {
            length: chunk_length,
            r#type: chunk_type,
            data: chunk_data,
            crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Chunk {
    pub fn length(&self) -> u32 {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    // #[test]
    // fn test_chunk_type() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    // }

    // #[test]
    // fn test_chunk_string() {
    //     let chunk = testing_chunk();
    //     let chunk_string = chunk.data_as_string().unwrap();
    //     let expected_chunk_string = String::from("This is where your secret message will be!");
    //     assert_eq!(chunk_string, expected_chunk_string);
    // }

    // #[test]
    // fn test_chunk_crc() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.crc(), 2882656334);
    // }

    // #[test]
    // fn test_valid_chunk_from_bytes() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;

    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();

    //     let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

    //     let chunk_string = chunk.data_as_string().unwrap();
    //     let expected_chunk_string = String::from("This is where your secret message will be!");

    //     assert_eq!(chunk.length(), 42);
    //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    //     assert_eq!(chunk_string, expected_chunk_string);
    //     assert_eq!(chunk.crc(), 2882656334);
    // }

    // #[test]
    // fn test_invalid_chunk_from_bytes() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656333;

    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();

    //     let chunk = Chunk::try_from(chunk_data.as_ref());

    //     assert!(chunk.is_err());
    // }

    // #[test]
    // pub fn test_chunk_trait_impls() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;

    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();
    //     let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
    //     let _chunk_string = format!("{}", chunk);
    // }
}
