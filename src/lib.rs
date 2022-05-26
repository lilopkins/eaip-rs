#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Parts of an eAIP.
pub mod parts;

/// Tools to work with eAIP publications.
pub mod eaip;

/// Parsers for different sections of the eAIP.
pub mod parse;

/// Generic data types used by this crate.
pub mod types;

/// Error type
pub mod error;

/// A convenience module that imports many useful parts of this crate.
pub mod prelude {
    pub use crate::parse::airports::Airports;
    pub use crate::parse::airways::Airways;
    pub use crate::parse::intersections::Intersections;
    pub use crate::parse::navaids::Navaids;
    pub use crate::parse::{FromEAIP, Parser};

    pub use crate::eaip::*;
    pub use crate::error::*;
    pub use crate::parts::*;
    pub use crate::types::*;
}
