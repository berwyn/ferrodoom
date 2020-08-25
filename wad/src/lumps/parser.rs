use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{error::LumpParseError, lumps::LumpType, Lump};

pub(crate) struct LumpParser<'a> {
    data: &'a [u8],
}

impl<'a> LumpParser<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub(crate) fn build(self, r#type: LumpType) -> crate::Result<Box<dyn Lump>> {
        if self.data.len() == 0 {
            tracing::warn!("LumpParser received an empty slice!");

            return Err(LumpParseError::NoData.into());
        }

        match r#type {
            LumpType::Nodes => self.parse_node(),
            LumpType::Unknown => Err(LumpParseError::UnknownType.into()),
        }
    }

    fn parse_node(self) -> crate::Result<Box<dyn Lump>> {
        let mut cursor = Cursor::new(self.data);

        let x_coord = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let y_coord = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let x_delta = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let y_delta = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;

        let right_top = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let right_bottom = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let right_left = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let right_right = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;

        let left_top = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let left_bottom = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let left_left = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let left_right = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;

        let right_child = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;
        let left_child = cursor
            .read_u16::<LittleEndian>()
            .map_err(LumpParseError::from)?;

        Ok(Box::new(crate::lumps::node::Node {
            partition_line: (x_coord, y_coord),
            partition_line_delta: (x_delta, y_delta),
            right_bounding_box: (right_top, right_bottom, right_left, right_right),
            left_bounding_box: (left_top, left_bottom, left_left, left_right),
            right_child,
            left_child,
        }))
    }
}

impl<'a> std::fmt::Debug for LumpParser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(LumpParser))
            .field("data", &format!("Vec<u8> ({} bytes)", self.data.len()))
            .finish()
    }
}
