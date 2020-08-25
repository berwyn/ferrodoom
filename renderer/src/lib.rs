mod bsp;
mod error;
mod renderer;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub use crate::renderer::Renderer;
