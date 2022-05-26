/// A result type, using the [`Error`] enum.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for eAIP errors
#[derive(Debug)]
pub enum Error {
    /// An error fetching data from an eAIP
    EAIPFetchError(reqwest::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EAIPFetchError(e) => {
                write!(f, "There was an error fetching data from the AIP: {}", e)
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::EAIPFetchError(e)
    }
}
