use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct ChunkType(u8, u8, u8, u8);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        ChunkType::new(&bytes)
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        ChunkType::new(bytes)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

impl ChunkType {
    const BYTE_LEN: u8 = 4;

    pub fn new(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != Self::BYTE_LEN.into() {
            return Err("[ChunkType::new] not given 4 bytes");
        }

        let are_bytes_ascii = bytes
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_uppercase());

        match are_bytes_ascii {
            true => Ok(ChunkType(bytes[0], bytes[1], bytes[2], bytes[3])),
            false => Err("[ChunkType::new] given bytes are not ascii"),
        }
    }

    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        todo!()
    }

    pub fn is_public(&self) -> bool {
        todo!()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.2.is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    // #[test]
    // pub fn test_chunk_type_is_critical() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_critical());
    // }

    // #[test]
    // pub fn test_chunk_type_is_not_critical() {
    //     let chunk = ChunkType::from_str("ruSt").unwrap();
    //     assert!(!chunk.is_critical());
    // }

    // #[test]
    // pub fn test_chunk_type_is_public() {
    //     let chunk = ChunkType::from_str("RUSt").unwrap();
    //     assert!(chunk.is_public());
    // }

    // #[test]
    // pub fn test_chunk_type_is_not_public() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(!chunk.is_public());
    // }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    // #[test]
    // pub fn test_chunk_type_is_safe_to_copy() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_safe_to_copy());
    // }

    // #[test]
    // pub fn test_chunk_type_is_unsafe_to_copy() {
    //     let chunk = ChunkType::from_str("RuST").unwrap();
    //     assert!(!chunk.is_safe_to_copy());
    // }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    // #[test]
    // pub fn test_chunk_type_string() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert_eq!(&chunk.to_string(), "RuSt");
    // }

    // #[test]
    // pub fn test_chunk_type_trait_impls() {
    //     let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //     let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //     let _chunk_string = format!("{}", chunk_type_1);
    //     let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    // }
}
