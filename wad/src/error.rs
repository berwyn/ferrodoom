use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    WadParse(#[from] WadParseError),
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
