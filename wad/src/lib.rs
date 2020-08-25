mod error;
mod lumps;
mod parser;
mod wad;

pub use crate::error::Error;
pub use crate::lumps::{node::Node, Lump};
pub use crate::parser::WadParser;
pub use crate::wad::{Wad, WadType};

pub type Result<T> = std::result::Result<T, error::Error>;
