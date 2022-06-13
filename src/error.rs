use std::fmt::Debug;

use airac::AIRAC;

use crate::parts::{EAIPType, Part};

/// A result type, using the [`Error`] enum.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for eAIP errors
#[derive(Debug)]
pub enum Error {
    /// An error fetching data from an eAIP
    EAIPFetchError(reqwest::Error),
    /// A page is missing from the eAIP
    EAIPMissingPage(AIRAC, Part, EAIPType),
    /// An error parsing the base_url of an EAIP when canonicalising URLs
    EAIPInvalidBaseURL(url::ParseError),
    /// An error while canonicalising URLs in joining the base URL to the chart URL
    ChartURLMalformed(String),
    /// The data cannot be scraped for the reason given.
    CannotScrapeData(&'static str),
    /// Some data cannot be parsed. The argument says what data.
    ParseError(&'static str, String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EAIPFetchError(e) => {
                write!(f, "There was an error fetching data from the AIP: {:?}", e)
            }
            Self::EAIPMissingPage(airac, part, typ) => write!(
                f,
                "The AIP does not have a page for {} {} {}",
                airac, part, typ
            ),
            Self::EAIPInvalidBaseURL(e) => {
                write!(f, "There was an error parsing the AIP base URL: {}", e)
            }
            Self::ChartURLMalformed(url) => {
                write!(f, "There was an error convering the chart URL: {}", url)
            }
            Self::CannotScrapeData(reason) => {
                write!(f, "The data cannot be scraped because {}", reason)
            }
            Self::ParseError(what, thing) => {
                write!(f, "The {} cannot be parsed ({:?}).", what, thing)
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::EAIPFetchError(e)
    }
}
