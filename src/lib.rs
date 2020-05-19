mod generes;

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ID3V1Error;

impl fmt::Display for ID3V1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Non ID3V1 file")
    }
}

impl Error for ID3V1Error {
    fn description(&self) -> &str {
        "Non ID3V1 file"
    }
}

#[derive(Debug)]
pub struct ID3V1Tag {
    pub song_name: String,
    pub artist: String,
    pub album_name: String,
    pub year: String,
    pub comment: String,
    pub genere: String,
}

impl ID3V1Tag {
    pub fn new(data: &[u8]) -> Result<ID3V1Tag, ID3V1Error> {
        if data.len() < 128 {
            return Err(ID3V1Error {});
        }
        if String::from_utf8_lossy(&data[0..3]) != "TAG" {
            return Err(ID3V1Error {});
        }
        Ok(ID3V1Tag {
            song_name: String::from_utf8_lossy(&data[3..33]).trim().to_owned(),
            artist: String::from_utf8_lossy(&data[33..63]).trim().to_owned(),
            album_name: String::from_utf8_lossy(&data[63..93]).trim().to_owned(),
            year: String::from_utf8_lossy(&data[93..97]).trim().to_owned(),
            comment: String::from_utf8_lossy(&data[97..127]).trim().to_owned(),
            genere: generes::TYPES[data[127] as usize].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_id3v1_tag_new_invalid_tag() {
        let data: [u8; 128] = [b'X'; 128];
        assert_eq!(
            ID3V1Tag::new(&data).unwrap_err(),
            ID3V1Error,
            "Should return ID3V1Error"
        );
    }

    #[test]
    fn test_id3v1_tag_new_invalid_size_buffer() {
        let data: [u8; 10] = [b'X'; 10];
        assert_eq!(
            ID3V1Tag::new(&data).unwrap_err(),
            ID3V1Error,
            "Should return ID3V1Error"
        );
    }
}
