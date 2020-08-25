mod error;
mod lumps;
mod parser;
mod wad;

pub use error::Error;
pub use parser::WadParser;
pub use wad::{Wad, WadType};

pub type Result<T> = std::result::Result<T, error::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
