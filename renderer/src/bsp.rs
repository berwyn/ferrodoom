use wad::{Node, Wad};

pub struct BSPNode {
    lump: Node,
    children: Vec<BSPNode>,
}

pub struct BSP {
    root: BSPNode,
}

impl BSP {
    fn build(&mut self, wad: Wad) {
        todo!()
    }

    fn walk(&self) {
        todo!()
    }
}
