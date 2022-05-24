use airac::{Datelike, AIRAC};
use std::fmt::Display;

/// The type of file to get from the eAIP
pub enum Type {
    /// An HTML file
    HTML,
    /// A PDF file
    PDF,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::HTML => "html",
                Self::PDF => "pdf",
            }
        )
    }
}

/// The parts of an AIP.
pub enum Part {
    /// General (GEN)
    General(GEN),
    /// En-Route (ENR)
    EnRoute(ENR),
    /// Aerodromes (AD)
    Aerodromes(AD),
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::General(g) => write!(f, "GEN-{}", g),
            Self::EnRoute(e) => write!(f, "ENR-{}", e),
            Self::Aerodromes(a) => write!(f, "AD-{}", a),
        }
    }
}

/// The General parts of an AIP
pub enum GEN {
    /// GEN 0
    Overview(usize),
    /// GEN 1 National Regulations and Requirements
    NationalRegulations(usize),
    /// GEN 2 Tables and Codes
    TablesAndCodes(usize),
    /// GEN 3 Services
    Services(usize),
    /// GEN 4 Charges for Aerodromes/Heliports and Air Navigation Services
    Charges(usize),
}

impl Display for GEN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Overview(page) => write!(f, "0.{}", page),
            Self::NationalRegulations(page) => write!(f, "1.{}", page),
            Self::TablesAndCodes(page) => write!(f, "2.{}", page),
            Self::Services(page) => write!(f, "3.{}", page),
            Self::Charges(page) => write!(f, "4.{}", page),
        }
    }
}

/// The En-Route parts of an AIP
pub enum ENR {
    /// ENR 0
    TableOfContents,
    /// ENR 1 General Rules and Procedures
    GeneralRules(usize),
    /// ENR 2 ATS Airspace
    ATSAirspace(usize),
    /// ENR 3 ATS Routes
    ATSRoutes(usize),
    /// ENR 4 Radio Navigation Aids and Systems
    RadioNavAids(usize),
    /// ENR 5 Navigation Warnings
    NavWarnings(usize),
    /// ENR 6 En-Route Charts
    Charts,
}

impl Display for ENR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::TableOfContents => write!(f, "0.1"),
            Self::GeneralRules(page) => write!(f, "1.{}", page),
            Self::ATSAirspace(page) => write!(f, "2.{}", page),
            Self::ATSRoutes(page) => write!(f, "3.{}", page),
            Self::RadioNavAids(page) => write!(f, "4.{}", page),
            Self::NavWarnings(page) => write!(f, "5.{}", page),
            Self::Charts => write!(f, "6"),
        }
    }
}

/// The Aerodromes parts of an AIP
pub enum AD {
    /// AD 0
    TableOfContents,
    /// AD 1 Aerodromes/Heliports -- Introduction
    Introduction(usize),
    /// AD 2 Aerodromes
    Aerodromes(String),
    /// AD 3 Heliports
    Heliports(String),
}

impl Display for AD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::TableOfContents => write!(f, "0.1"),
            Self::Introduction(page) => write!(f, "1.{}", page),
            Self::Aerodromes(icao) => write!(f, "2.{}", icao),
            Self::Heliports(icao) => write!(f, "3.{}", icao),
        }
    }
}

/// Generate the location in an eAIP package for a particular section
pub fn generate_location<S: AsRef<str> + Display>(
    country_code: S,
    section: Part,
    locale: S,
    typ: Type,
) -> String {
    format!(
        "/{}/eAIP/{}-{}-{}.{}",
        typ, country_code, section, locale, typ
    )
}

/// Generate the location in an eAIP package for a particular section
pub fn generate_location_with_airac<S: AsRef<str> + Display>(
    airac: AIRAC,
    country_code: S,
    section: Part,
    locale: S,
    typ: Type,
) -> String {
    format!(
        "/{:04}-{:02}-{:02}-AIRAC{}",
        airac.starts().year(),
        airac.starts().month(),
        airac.starts().day(),
        generate_location(country_code, section, locale, typ)
    )
}

#[cfg(test)]
mod tests {
    use crate::parts::*;

    #[test]
    fn test_generate_location() {
        assert_eq!(
            "/html/eAIP/EG-AD-2.EGBO-en-GB.html",
            generate_location(
                "EG",
                Part::Aerodromes(AD::Aerodromes("EGBO".to_string())),
                "en-GB",
                Type::HTML
            )
        );
    }
}
