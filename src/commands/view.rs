use crate::id3v1::ID3V1Tag;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

pub fn view(fname: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(fname)?;
    file.seek(SeekFrom::End(-128))?;
    let mut buffer = [0; 128];
    file.read_exact(&mut buffer)?;

    let tag = ID3V1Tag::from(&buffer)?;
    println!("tag {:?}", tag);

    Ok(())
}
