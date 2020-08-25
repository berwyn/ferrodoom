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

pub enum NamedLumps {
    Things,
    Linedefs,
    Sidedefs,
    Vertices,
    Segments,
    Subsectors,
    Nodes,
    Sectors,
    Reject,
    Blockmap,
    Behavior,
}

impl std::string::ToString for NamedLumps {
    fn to_string(&self) -> String {
        match self {
            NamedLumps::Things => "THINGS",
            NamedLumps::Linedefs => "LINEDEFS",
            NamedLumps::Sidedefs => "SIDEDEFS",
            NamedLumps::Vertices => "VERTEXES",
            NamedLumps::Segments => "SEGS",
            NamedLumps::Subsectors => "SSECTORS",
            NamedLumps::Nodes => "NODES",
            NamedLumps::Sectors => "SECTORS",
            NamedLumps::Reject => "REJECT",
            NamedLumps::Blockmap => "BLOCKMAP",
            NamedLumps::Behavior => "BEHAVIOR",
        }
        .into()
    }
}

pub trait Lump: std::fmt::Debug {}
