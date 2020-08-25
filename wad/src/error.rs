use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    WadParse(#[from] WadParseError),
    #[error(transparent)]
    LumpParse(#[from] LumpParseError),
}

#[derive(Debug, Error)]
pub enum WadParseError {
    #[error("Invalid WAD type")]
    InvalidType,
    #[error("Data isn't valid")]
    InvalidHeaderData,
    #[error("The directory entry is invalid")]
    InvalidDirectoryEntry,
}

#[derive(Debug, Error)]
pub enum LumpParseError {
    #[error("No data provided")]
    NoData,
    #[error("Unknown lump type")]
    UnknownType,
    #[error("Invalid entry")]
    InvalidEntry(#[from] std::io::Error),
}
