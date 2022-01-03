use std::fs;
use std::str::FromStr;

use crate::args;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

// TODO can we let caller handle errors? redundant expect() everywhere

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: args::EncodeArgs) {
    // read bytes from file and create Png
    let mut png = Png::from_file(&args.filename).expect("Unable to create Png");

    // create new chunk
    let chunk_type = ChunkType::from_str(&args.chunk_type).expect("Invalid chunk type");
    let chunk_data: Vec<u8> = args.message.bytes().collect();
    let chunk = Chunk::new(chunk_type, chunk_data);

    // TODO validate chunk (here or in chunk.rs?) - can't be critical or sth like that idk
    png.append_chunk(chunk);

    // write new Png bytes into file
    fs::write(&args.filename, png.as_bytes()).expect("Unable to write file");
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: args::DecodeArgs) {
    let png = Png::from_file(&args.filename).expect("Unable to create Png");

    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => println!(
            "{}",
            chunk
                .data_as_string()
                .expect("Unable to read data in chunk")
        ),
        None => println!("Couldn't find chunk with given chunk type!"),
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: args::RemoveArgs) {
    let mut png = Png::from_file(&args.filename).expect("Unable to create Png");

    match png.remove_chunk(&args.chunk_type) {
        Ok(_chunk) => {
            fs::write(&args.filename, png.as_bytes()).expect("Unable to write file");
            println!("Successfully removed chunk");
        }
        Err(_) => eprintln!("Failed to remove chunk"),
    }
}

// TODO add an option for verbose print
/// Prints all of the chunks in a PNG file
pub fn print(args: args::PrintArgs) {
    let png = Png::from_file(&args.filename).expect("Unable to create Png");
    println!("{}", png);
}
