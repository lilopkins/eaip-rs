use chrono::Duration;
use lazy_static::lazy_static;

use super::*;

/// EAIP with a nice name
pub struct NamedEAIP {
    country: &'static str,
    name: &'static str,
    icao_prefix: &'static str,
    url: &'static str,
    eaip: EAIP,
}

impl NamedEAIP {
    /// ISO3166 two letter country code for this AIS, in capitals.
    pub fn country(&self) -> &'static str {
        self.country
    }

    /// Presentable source name for this AIS.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// AIS source.
    pub fn eaip(&self) -> &EAIP {
        &self.eaip
    }

    /// ICAO prefix. Every ICAO code within this AIS source starts with this.
    pub fn icao_prefix(&self) -> &'static str {
        self.icao_prefix
    }

    /// URL for this AIS source.
    pub fn url(&self) -> &'static str {
        self.url
    }
}

lazy_static! {
    /// All AISs in a big list.
    pub static ref ALL: Vec<&'static NamedEAIP> = vec![
        &GB,
        &NL,
    ];
    /// United Kingdom: [NATS](https://nats.aero)
    pub static ref GB: NamedEAIP = NamedEAIP {
        country: "GB",
        name: "NATS",
        icao_prefix: "EG",
        url: "https://nats.aero",
        eaip: EAIP::new("https://www.aurora.nats.co.uk/htmlAIP/Publications", "EG", "en-GB")
    };
    /// The Netherlands: [LVNL](https://lvnl.nl)
    pub static ref NL: NamedEAIP = NamedEAIP {
        country: "NL",
        name: "LVNL",
        icao_prefix: "EH",
        url: "https://lvnl.nl",
        eaip: EAIP::new_with_offset("https://eaip.lvnl.nl", "EH", "en-GB", Duration::weeks(2))
    };
}
