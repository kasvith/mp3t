use std::fs::File;
use std::io::prelude::*;
use std::io::*;

use mp3t::ID3V1Tag;

fn main() {
    let mut file = File::open("song.mp3").unwrap();
    let pos = file.seek(SeekFrom::End(-128)).unwrap();
    println!("pos {}", pos);
    let mut buffer = [0; 128];
    file.read_exact(&mut buffer).unwrap();

    let tag = ID3V1Tag::new(&buffer).unwrap();
    println!("tag {:?}", tag);
}
