use airac::AIRAC;
use chrono::Duration;

use crate::{error::Error, parts::*};

/// Predefined [`EAIP`] objects for global aeronautical information services.
pub mod ais;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Details to an online aeronautical information publication
pub struct EAIP {
    base_uri: String,
    country_code: String,
    locale: String,
    publish_offset: Duration,
}

impl EAIP {
    /// Create a new aeronautical information publication.
    /// Note that some of the global AIP providers can be found in the `ais` module by enabling the `ais` feature.
    pub fn new<S: Into<String>>(base_uri: S, country_code: S, locale: S) -> Self {
        Self::new_with_offset(base_uri, country_code, locale, Duration::zero())
    }

    /// Create a new aeronautical information publication with a publishing offset.
    /// If the eAIP is published two weeks in advance, the offset would be `Duration::weeks(2)`.
    /// Note that some of the global AIP providers can be found in the `ais` module by enabling the `ais` feature.
    pub fn new_with_offset<S: Into<String>>(
        base_uri: S,
        country_code: S,
        locale: S,
        publish_offset: Duration,
    ) -> Self {
        Self {
            base_uri: base_uri.into(),
            country_code: country_code.into(),
            locale: locale.into(),
            publish_offset,
        }
    }

    /// Get a page from an eAIP for the current AIRAC.
    pub async fn get_current_page(&self, part: Part, typ: EAIPType) -> Result<String, Error> {
        let airac = AIRAC::current();
        self.get_page(airac, part, typ).await
    }

    /// Get a page from an eAIP for the specified AIRAC.
    pub async fn get_page(&self, airac: AIRAC, part: Part, typ: EAIPType) -> Result<String, Error> {
        let url = self.generate_url(airac.clone(), part.clone(), typ);
        log::debug!("Getting page: {}", url);
        let res = reqwest::get(&url).await?;
        if res.status().is_success() {
            return Ok(res.text().await?);
        }
        Err(Error::EAIPMissingPage(airac, part, typ))
    }

    /// Generate a URL within this eAIP
    pub fn generate_url(&self, airac: AIRAC, part: Part, typ: EAIPType) -> String {
        format!(
            "{}{}",
            self.base_uri,
            generate_location_with_airac_and_offseet(
                airac,
                self.publish_offset,
                self.country_code.clone(),
                part,
                self.locale.clone(),
                typ
            )
        )
    }
}
