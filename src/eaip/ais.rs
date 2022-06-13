use chrono::Duration;
use lazy_static::lazy_static;

use super::*;

/// EAIP with a nice name
pub struct NamedEAIP {
    country: &'static str,
    name: &'static str,
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
        eaip: EAIP::new("https://www.aurora.nats.co.uk/htmlAIP/Publications", "EG", "en-GB")
    };
    /// The Netherlands: [LVNL](https://lvnl.nl)
    pub static ref NL: NamedEAIP = NamedEAIP {
        country: "NL",
        name: "LVNL",
        eaip: EAIP::new_with_offset("https://eaip.lvnl.nl", "EH", "en-GB", Duration::weeks(2))
    };
}
