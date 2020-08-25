use wad::{Lump, Wad};

pub struct BSPNode {
    lump: Box<dyn Lump>,
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
