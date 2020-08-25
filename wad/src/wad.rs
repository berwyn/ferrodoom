use std::collections::HashMap;

use crate::{lumps::parser::LumpParser, Lump};

#[derive(Debug, Eq, PartialEq)]
pub enum WadType {
    Internal,
    Patch,
}

#[derive(Debug)]
pub(crate) struct WadDirectoryEntry {
    pub(crate) position: u32,
    pub(crate) size: u32,
    pub(crate) name: String,
}

pub(crate) type WadDirectory = HashMap<String, WadDirectoryEntry>;

pub struct Wad {
    pub(crate) data: Vec<u8>,
    pub(crate) kind: WadType,
    pub(crate) lump_count: u32,
    pub(crate) directory_location: u32,
    pub(crate) directory: WadDirectory,
}

impl Wad {
    pub fn find(&self, name: &str) -> Option<Box<dyn Lump>> {
        tracing::debug!("Seeking {}", name);

        let entry = self.directory.get(name)?;

        tracing::debug!("Found entry for {}", name);
        tracing::debug!("Seeking to {}", &entry.position);

        if entry.size == 0 {
            return Some(Box::new(crate::lumps::marker::Virtual));
        }

        let header_size = 12 as usize;
        let position = header_size + entry.position as usize;
        let end_position = position + entry.size as usize;
        let range = position..end_position;

        tracing::debug!(
            "Building slice from {} to {}: {:?}",
            position,
            end_position,
            range
        );

        LumpParser::new(&self.data[range]).build(name.into()).ok()
    }
}

impl std::fmt::Debug for Wad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Wad))
            .field("kind", &self.kind)
            .field("lump_count", &self.lump_count)
            .field("directory_location", &self.directory_location)
            .field("directory", &self.directory)
            .field("data", &format!("Vec<u8> ({} bytes)", self.data.len()))
            .finish()
    }
}
