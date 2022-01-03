use std::fs;
use std::str::FromStr;

use crate::args;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::{Error, Result};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: args::EncodeArgs) -> Result<()> {
    // read bytes from file and create Png
    let mut png = Png::from_file(&args.filename)?;

    // create new chunk
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk_data: Vec<u8> = args.message.bytes().collect();
    let chunk = Chunk::new(chunk_type, chunk_data);

    // TODO validate chunk (here or in chunk.rs?) - can't be critical or sth like that idk
    png.append_chunk(chunk);

    // write new Png bytes into file
    fs::write(&args.filename, png.as_bytes())?;

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: args::DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.filename)?;

    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => println!("{}", chunk.data_as_string()?), // TODO would like to return custom error msg here. How to do that cleanly?
        None => return Err("Couldn't find chunk with given chunk type".into()),
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: args::RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.filename)?;

    match png.remove_chunk(&args.chunk_type) {
        Ok(_chunk) => {
            fs::write(&args.filename, png.as_bytes())?;
            println!("Successfully removed chunk");
        }
        Err(_) => return Err("Failed to remove chunk".into()),
    }

    Ok(())
}

// TODO add an option for verbose print
/// Prints all of the chunks in a PNG file
pub fn print(args: args::PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.filename)?;
    println!("{}", png);

    Ok(())
}
