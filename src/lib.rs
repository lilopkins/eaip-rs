#![warn(missing_docs)]

//! # eAIP Parse
//!
//! A crate containing tools to parse HTML eAIPs, as XML versions are not often published.

/// Parts of an eAIP
pub mod parts;

/// Tools to work with eAIP publications.
pub mod eaip;

/// Parsers for different sections of the eAIP.
pub mod parse;

/// Generic data types used by this crate.
pub mod types;

/// A convenience module that imports many useful parts of this crate.
pub mod prelude {
    pub use crate::parse::navaids::Navaids;
    pub use crate::parse::Parser;

    pub use crate::types::*;
}
