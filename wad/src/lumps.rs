pub mod linedef;
pub mod marker;
pub mod node;
pub mod sector;
pub mod seg;
pub mod sidedef;
pub mod subsector;
pub mod thing;
pub mod vertex;

pub(crate) mod parser;

pub(crate) enum LumpType {
    Nodes,
    Unknown,
}

impl From<&str> for LumpType {
    fn from(s: &str) -> Self {
        match s {
            "NODES" => LumpType::Nodes,
            _ => LumpType::Unknown,
        }
    }
}

pub trait Lump: std::fmt::Debug {}
