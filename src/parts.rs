use airac::{Datelike, AIRAC};
use std::fmt::Display;

/// The type of file to get from the eAIP
pub enum EAIPType {
    /// An HTML file
    HTML,
    /// A PDF file
    PDF,
}

impl Display for EAIPType {
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
    typ: EAIPType,
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
    typ: EAIPType,
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

    macro_rules! test_route {
        ($uri: expr, $route: expr) => {
            assert_eq!(
                $uri,
                generate_location("EG", $route, "en-GB", EAIPType::HTML)
            );
        };
    }

    #[test]
    fn test_generate_location() {
        test_route!(
            "/html/eAIP/EG-AD-2.EGBO-en-GB.html",
            Part::Aerodromes(AD::Aerodromes("EGBO".to_string()))
        );
        test_route!(
            "/html/eAIP/EG-GEN-0.1-en-GB.html",
            Part::General(GEN::Overview(1))
        );
        test_route!(
            "/html/eAIP/EG-ENR-0.1-en-GB.html",
            Part::EnRoute(ENR::TableOfContents)
        );
    }

    #[test]
    fn test_general() {
        test_route!(
            "/html/eAIP/EG-GEN-0.1-en-GB.html",
            Part::General(GEN::Overview(1))
        );
        test_route!(
            "/html/eAIP/EG-GEN-1.1-en-GB.html",
            Part::General(GEN::NationalRegulations(1))
        );
        test_route!(
            "/html/eAIP/EG-GEN-2.1-en-GB.html",
            Part::General(GEN::TablesAndCodes(1))
        );
        test_route!(
            "/html/eAIP/EG-GEN-3.1-en-GB.html",
            Part::General(GEN::Services(1))
        );
        test_route!(
            "/html/eAIP/EG-GEN-4.1-en-GB.html",
            Part::General(GEN::Charges(1))
        );
    }

    #[test]
    fn test_enroute() {
        test_route!(
            "/html/eAIP/EG-ENR-0.1-en-GB.html",
            Part::EnRoute(ENR::TableOfContents)
        );
        test_route!(
            "/html/eAIP/EG-ENR-1.1-en-GB.html",
            Part::EnRoute(ENR::GeneralRules(1))
        );
        test_route!(
            "/html/eAIP/EG-ENR-2.1-en-GB.html",
            Part::EnRoute(ENR::ATSAirspace(1))
        );
        test_route!(
            "/html/eAIP/EG-ENR-3.1-en-GB.html",
            Part::EnRoute(ENR::ATSRoutes(1))
        );
        test_route!(
            "/html/eAIP/EG-ENR-4.1-en-GB.html",
            Part::EnRoute(ENR::RadioNavAids(1))
        );
        test_route!(
            "/html/eAIP/EG-ENR-5.1-en-GB.html",
            Part::EnRoute(ENR::NavWarnings(1))
        );
        test_route!("/html/eAIP/EG-ENR-6-en-GB.html", Part::EnRoute(ENR::Charts));
    }

    #[test]
    fn test_aerodromes() {
        test_route!(
            "/html/eAIP/EG-AD-0.1-en-GB.html",
            Part::Aerodromes(AD::TableOfContents)
        );
        test_route!(
            "/html/eAIP/EG-AD-1.1-en-GB.html",
            Part::Aerodromes(AD::Introduction(1))
        );
        test_route!(
            "/html/eAIP/EG-AD-2.ZZZZ-en-GB.html",
            Part::Aerodromes(AD::Aerodromes("ZZZZ".to_string()))
        );
        test_route!(
            "/html/eAIP/EG-AD-3.ZZZZ-en-GB.html",
            Part::Aerodromes(AD::Heliports("ZZZZ".to_string()))
        );
    }

    #[test]
    fn test_generate_location_pdf() {
        assert_eq!(
            "/pdf/eAIP/EG-AD-2.EGBO-en-GB.pdf",
            generate_location(
                "EG",
                Part::Aerodromes(AD::Aerodromes("EGBO".to_string())),
                "en-GB",
                EAIPType::PDF
            )
        );
    }

    #[test]
    fn test_generate_location_with_airac() {
        assert_eq!(
            "/2022-05-19-AIRAC/html/eAIP/EG-AD-2.EGBO-en-GB.html",
            generate_location_with_airac(
                airac::AIRAC::from_ymd(2022, 05, 19),
                "EG",
                Part::Aerodromes(AD::Aerodromes("EGBO".to_string())),
                "en-GB",
                EAIPType::HTML
            )
        );
    }
}
