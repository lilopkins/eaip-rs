use lazy_static::lazy_static;

use super::*;

lazy_static! {
    /// United Kingdom: [NATS](https://nats.aero)
    pub static ref GB: EAIP<'static> = EAIP::new("https://www.aurora.nats.co.uk/htmlAIP/Publications", "EG", "en-GB");
    /// The Netherlands: [LVNL](https://lvnl.nl)
    pub static ref NL: EAIP<'static> = EAIP::new("https://eaip.lvnl.nl", "EH", "en-GB");
}
