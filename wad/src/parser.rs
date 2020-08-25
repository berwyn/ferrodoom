use std::{collections::HashMap, io::Cursor};

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use pad::PadStr;

use crate::{
    error::WadParseError,
    wad::{Wad, WadDirectory, WadDirectoryEntry, WadType},
};

pub struct WadParser {
    data: Vec<u8>,
}

impl WadParser {
    pub fn with_data(mut self, data: impl Clone + AsRef<[u8]>) -> Self {
        self.data = data.as_ref().into();
        self
    }

    pub fn parse(self) -> crate::Result<Wad> {
        let kind = self.parse_wad_type()?;
        let lump_count = self.parse_lump_count();
        let directory_location = self.parse_directory_pointer();
        let directory = self.parse_directory(directory_location, lump_count)?;

        Ok(Wad {
            kind,
            lump_count,
            directory_location,
            directory,
        })
    }

    fn parse_wad_type(&self) -> crate::Result<WadType> {
        match String::from_utf8(self.data[0x00..0x04].to_ascii_uppercase()) {
            Ok(text) => match text.as_str() {
                "IWAD" => Ok(WadType::Internal),
                "PWAD" => Ok(WadType::Patch),
                _ => Err(WadParseError::InvalidType.into()),
            },
            Err(_) => Err(WadParseError::InvalidHeaderData.into()),
        }
    }

    fn parse_lump_count(&self) -> u32 {
        let sequence = &self.data[0x04..0x08];
        LittleEndian::read_u32(sequence)
    }

    fn parse_directory_pointer(&self) -> u32 {
        let sequence = &self.data[0x08..0x0C];
        LittleEndian::read_u32(sequence)
    }

    fn parse_directory(&self, location: u32, count: u32) -> crate::Result<WadDirectory> {
        let directory_end = location + (count * 16);
        let index = (location as usize)..(directory_end as usize);
        let mut directory: WadDirectory = HashMap::new();

        for chunk in self.data[index].chunks(16) {
            let mut cursor = Cursor::new(chunk);
            let position = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| WadParseError::InvalidDirectoryEntry)?;
            let size = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| WadParseError::InvalidDirectoryEntry)?;
            let name = String::from_utf8(chunk[0x08..].into())
                .map_err(|_| WadParseError::InvalidDirectoryEntry)?;

            let entry = WadDirectoryEntry {
                position,
                size,
                name: name.clone(),
            };

            directory.insert(name, entry);
        }

        Ok(directory)
    }
}

impl Default for WadParser {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::WriteBytesExt;
    use pretty_assertions::assert_eq;
    use std::io::Write;

    fn generate_wad_header() -> Cursor<Vec<u8>> {
        let bytes: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(bytes);

        cursor.write_all("IWAD".as_bytes()).unwrap();
        cursor.write_i32::<LittleEndian>(0x01).unwrap();
        cursor.write_i32::<LittleEndian>(0x0C).unwrap();
        cursor.write_u32::<LittleEndian>(0x100).unwrap();
        cursor.write_u32::<LittleEndian>(0x200).unwrap();
        cursor.write_all("TEST1234".as_bytes()).unwrap();
        cursor.flush().unwrap();
        cursor.set_position(0);
        cursor
    }

    #[test]
    fn it_parses_wad_type() {
        let parser = WadParser::default().with_data("IWAD".as_bytes());
        let wad_type = parser.parse_wad_type().unwrap();

        assert_eq!(WadType::Internal, wad_type);

        let parser = WadParser::default().with_data("PWAD".as_bytes());
        let wad_type = parser.parse_wad_type().unwrap();

        assert_eq!(WadType::Patch, wad_type);
    }

    #[test]
    fn it_parses_lump_count() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = generate_wad_header();
        let parser = WadParser::default().with_data(cursor.get_ref());
        let lump_count = parser.parse_lump_count();

        assert_eq!(0x01, lump_count);

        Ok(())
    }

    #[test]
    fn it_parses_the_directory_pointer() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = generate_wad_header();
        let parser = WadParser::default().with_data(cursor.get_ref());
        let directory_pointer = parser.parse_directory_pointer();

        assert_eq!(0x0C, directory_pointer);

        Ok(())
    }

    #[test]
    fn it_parses_directory_entries() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = generate_wad_header();
        let parser = WadParser::default().with_data(cursor.get_ref());
        let directory = parser.parse_directory(0x0C, 0x01)?;

        assert_eq!(1, directory.len());

        let entry = directory.get("TEST1234").unwrap();

        assert_eq!(0x100, entry.position);
        assert_eq!(0x200, entry.size);
        assert_eq!("TEST1234", entry.name);

        Ok(())
    }

    #[test]
    fn it_parses_a_wad() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = generate_wad_header();
        let parser = WadParser::default().with_data(cursor.get_ref());
        let wad = parser.parse()?;

        assert_eq!(WadType::Internal, wad.kind);
        assert_eq!(0x01, wad.lump_count);
        assert_eq!(0x0C, wad.directory_location);
        assert_eq!(1, wad.directory.len());

        Ok(())
    }
}
