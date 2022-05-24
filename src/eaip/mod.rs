use airac::AIRAC;

use crate::parts::*;

/// Predefined [`EAIP`] objects for global aeronautical information services.
pub mod ais;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Details to an online aeronautical information publication
pub struct EAIP<'a> {
    base_uri: &'a str,
    country_code: &'a str,
    locale: &'a str,
}

impl<'a> EAIP<'a> {
    /// Create a new aeronautical information publication.
    /// Note that some of the global AIP providers can be found in the `ais` module by enabling the `ais` feature.
    pub fn new(base_uri: &'a str, country_code: &'a str, locale: &'a str) -> Self {
        Self {
            base_uri,
            country_code,
            locale,
        }
    }

    /// Get a page from an eAIP for the current AIRAC.
    pub async fn get_current_page(&self, part: Part, typ: Type) -> Result<String, reqwest::Error> {
        let airac = AIRAC::current();
        Ok(self.get_page(airac, part, typ).await?)
    }

    /// Get a page from an eAIP for the specified AIRAC.
    pub async fn get_page(
        &self,
        airac: AIRAC,
        part: Part,
        typ: Type,
    ) -> Result<String, reqwest::Error> {
        let url = format!(
            "{}{}",
            self.base_uri,
            generate_location_with_airac(airac, self.country_code, part, self.locale, typ)
        );
        log::debug!("Getting page: {}", url);
        Ok(reqwest::get(url).await?.text().await?)
    }
}
