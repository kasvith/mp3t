use crate::id3v1::ID3V1Tag;
use dialoguer::Input;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

pub fn edit(fname: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).write(true).open(fname)?;
    file.seek(SeekFrom::End(-128))?;
    let mut buffer = [0; 128];
    file.read_exact(&mut buffer)?;
    let mut tag: ID3V1Tag;

    if String::from_utf8_lossy(&buffer[0..3]) != "TAG" {
        // seek to eof
        file.seek(SeekFrom::End(0))?;
        tag = ID3V1Tag::default();
    } else {
        tag = ID3V1Tag::from(&buffer)?;
    }

    tag.song_title = Input::<String>::new()
        .with_prompt("Song Title")
        .interact()?;
    tag.album = Input::<String>::new().with_prompt("Album").interact()?;
    let genere_id = dialoguer::Select::new()
        .items(crate::generes::TYPES)
        .paged(true)
        .with_prompt("genere")
        .interact()?;
    println!("{}", genere_id);

    file.write(&tag.to_buf())?;

    Ok(())
}
