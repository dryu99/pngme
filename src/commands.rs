use std::fs;
use std::str::FromStr;

use crate::args;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(args: args::EncodeArgs) {
    // read bytes from file and create Png
    let mut png = Png::from_file(&args.filename).expect("Unable to create Png");

    // create new chunk
    let chunk_type = ChunkType::from_str(&args.chunk_type).expect("Invalid chunk type");
    let chunk_data: Vec<u8> = args.message.bytes().collect();
    let chunk = Chunk::new(chunk_type, chunk_data);
    println!("chunk: {}", chunk);

    // TODO validate chunk (here or in chunk.rs?) - can't be critical or sth like that idk
    png.append_chunk(chunk);

    // write new Png bytes into file
    fs::write(&args.filename, png.as_bytes()).expect("Unable to write file");
}

pub fn decode(args: args::DecodeArgs) {
    todo!()
}

pub fn remove(args: args::RemoveArgs) {
    todo!()
}

pub fn print(args: args::PrintArgs) {
    let png = Png::from_file(&args.filename).expect("Unable to create Png");
    println!("{}", png);

    // TODO add an option for print for verbose output
    for chunk in png.chunks() {
        println!("chunk: {}", chunk);
    }
}
