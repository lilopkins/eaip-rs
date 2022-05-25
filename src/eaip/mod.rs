use airac::AIRAC;

use crate::parts::*;

/// Predefined [`EAIP`] objects for global aeronautical information services.
pub mod ais;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Details to an online aeronautical information publication
pub struct EAIP {
    base_uri: String,
    country_code: String,
    locale: String,
}

impl EAIP {
    /// Create a new aeronautical information publication.
    /// Note that some of the global AIP providers can be found in the `ais` module by enabling the `ais` feature.
    pub fn new<S: Into<String>>(base_uri: S, country_code: S, locale: S) -> Self {
        Self {
            base_uri: base_uri.into(),
            country_code: country_code.into(),
            locale: locale.into(),
        }
    }

    /// Get a page from an eAIP for the current AIRAC.
    pub async fn get_current_page(&self, part: Part, typ: EAIPType) -> Result<String, reqwest::Error> {
        let airac = AIRAC::current();
        Ok(self.get_page(airac, part, typ).await?)
    }

    /// Get a page from an eAIP for the specified AIRAC.
    pub async fn get_page(
        &self,
        airac: AIRAC,
        part: Part,
        typ: EAIPType,
    ) -> Result<String, reqwest::Error> {
        let url = self.generate_url(airac, part, typ);
        log::debug!("Getting page: {}", url);
        Ok(reqwest::get(url).await?.text().await?)
    }

    /// Generate a URL within this eAIP
    pub fn generate_url(&self, airac: AIRAC, part: Part, typ: EAIPType) -> String {
        format!(
            "{}{}",
            self.base_uri,
            generate_location_with_airac(airac, self.country_code.clone(), part, self.locale.clone(), typ)
        )
    }
}
