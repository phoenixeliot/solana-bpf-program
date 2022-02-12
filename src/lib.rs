// inside lib.rs, only the following line should be in here
pub mod error;
pub mod instruction;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
