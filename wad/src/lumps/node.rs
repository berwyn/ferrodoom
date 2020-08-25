use crate::Lump;

#[derive(Debug)]
pub struct Node {
    pub(crate) partition_line: (u16, u16),
    pub(crate) partition_line_delta: (u16, u16),
    pub(crate) right_bounding_box: (u16, u16, u16, u16),
    pub(crate) left_bounding_box: (u16, u16, u16, u16),
    pub(crate) right_child: u16,
    pub(crate) left_child: u16,
}

impl Lump for Node {}
