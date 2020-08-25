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
            LumpType::Nodes => todo!(),
            LumpType::Unknown => Err(LumpParseError::UnknownType.into()),
        }
    }
}

impl<'a> std::fmt::Debug for LumpParser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(LumpParser))
            .field("data", &format!("Vec<u8> ({} bytes)", self.data.len()))
            .finish()
    }
}
