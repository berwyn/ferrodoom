use std::collections::HashMap;

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

#[derive(Debug)]
pub struct Wad {
    pub(crate) kind: WadType,
    pub(crate) lump_count: u32,
    pub(crate) directory_location: u32,
    pub(crate) directory: WadDirectory,
}
