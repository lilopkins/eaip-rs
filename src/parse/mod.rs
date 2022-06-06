use std::collections::VecDeque;

use airac::AIRAC;
use async_trait::async_trait;
use ego_tree::iter::{Edge, Traverse};
use regex::Regex;
use scraper::{Html, Node};

use crate::prelude::*;

/// Parsers for a list of radio navaids.
pub mod navaids;

/// Parsers for a list of intersections.
pub mod intersections;

/// Parsers for a list of airways.
pub mod airways;

/// Parsers for airports.
pub mod airports;

/// Fetch and parse some data from an eAIP
#[async_trait]
pub trait FromEAIP {
    /// The type this parser will output when successful
    type Output;

    /// Fetch the data from the given eAIP for the given AIRAC.
    async fn from_eaip(eaip: &EAIP, airac: AIRAC) -> Result<Self::Output>;

    /// Fetch the data from the given eAIP for the current AIRAC.
    async fn from_current_eaip(eaip: &EAIP) -> Result<Self::Output> {
        Self::from_eaip(eaip, AIRAC::current()).await
    }
}

/// The trait for all eAIP data parsers
pub trait Parser<'a> {
    /// The type this parser will output when successful
    type Output;

    /// Parse the given HTML data into the type given by `Self::Output`.
    fn parse(data: &'a str) -> Result<Self::Output>;
}

/// Get just the text content of some HTML, removing elements that are hidden with display: none.
pub(crate) fn get_clean_text(html_frag: String) -> String {
    let frag = Html::parse_fragment(&html_frag);
    get_clean_text_traverse(frag.root_element().traverse())
        .trim()
        .to_string()
}

fn get_clean_text_traverse(traverse: Traverse<'_, Node>) -> String {
    let mut s = String::new();
    let mut ignore_chain = VecDeque::new();
    for edge in traverse {
        match edge {
            Edge::Open(node) => {
                match node.value() {
                    Node::Element(e) => {
                        // if <br>, newline
                        if e.name() == "br" || e.name() == "p" {
                            if !s.ends_with('\n') {
                                s += "\n";
                            }
                        } else if let Some(attr) = e.attr("style") {
                            // if <... style="display: none">, push_front to ignore chain
                            if attr == "display: none;" {
                                // TODO: Improve reliability
                                ignore_chain.push_front(e.name());
                            }
                        }
                    }
                    Node::Text(t) => {
                        if ignore_chain.is_empty() {
                            s += &*t.trim();
                        }
                    }
                    _ => (),
                }
            }
            Edge::Close(node) => {
                // if matches end of ignore chain, pop_front
                if let Node::Element(e) = node.value() {
                    if !s.ends_with('\n') {
                        let inline_elems = vec!["span", "strong", "i", "em"];
                        if !inline_elems.contains(&&*e.name().to_lowercase()) {
                            s += "\n";
                        }
                    }

                    if !ignore_chain.is_empty() && e.name() == ignore_chain[0] {
                        ignore_chain.pop_front().unwrap();
                    }
                }
            }
        }
    }
    s
}

/// Parses a frequency - always returns kHz
pub(crate) fn parse_frequency<S: Into<String>>(data: S) -> Result<usize> {
    let re = Regex::new(r"([0-9.]{3,7})\s*([kM])Hz").unwrap();
    let data = data.into();

    if let Some(caps) = re.captures(&data) {
        let mut freq = caps[1].parse::<f32>().unwrap();
        if &caps[2] == "M" {
            freq *= 1000f32;
        }
        Ok(freq as usize)
    } else {
        Err(Error::ParseError("frequency", data))
    }
}

/// Parses a latlong
pub(crate) fn parse_latlong<S: Into<String>>(data: S) -> Result<(Option<f64>, Option<f64>)> {
    let re = Regex::new(r"(?:([0-9.]{6,})([NnSs]))?\s*(?:([0-9.]{7,})([EeWw]))?").unwrap();
    let data = data.into();
    let mut lat = None;
    let mut lon = None;
    if let Some(caps) = re.captures(&data) {
        if let Some(raw_lat) = caps.get(1) {
            lat = Some(raw_lat.as_str().parse::<f64>().unwrap() / 10000f64);
            if caps[2].to_lowercase() == *"s" {
                lat = Some(-lat.unwrap());
            }
        }
        if let Some(raw_lon) = caps.get(3) {
            lon = Some(raw_lon.as_str().parse::<f64>().unwrap() / 10000f64);
            if caps[4].to_lowercase() == *"w" {
                lon = Some(-lon.unwrap());
            }
        }
    }

    if lat == None && lon == None {
        Err(Error::ParseError("latlong", data))
    } else {
        Ok((lat, lon))
    }
}

/// Parses an elevation, always returning ft
pub(crate) fn parse_elevation<S: Into<String>>(data: S) -> Result<usize> {
    let re = Regex::new(r"([0-9]+)\s*(?:ft|FT)").unwrap();
    let data = data.into();

    if let Some(caps) = re.captures(&data) {
        Ok(caps[1].parse::<usize>().unwrap())
    } else {
        Err(Error::ParseError("elevation", data))
    }
}

#[cfg(test)]
mod tests {
    use super::{get_clean_text, parse_elevation, parse_frequency, parse_latlong};

    #[test]
    fn test_already_clean_string() {
        assert_eq!("CLN", get_clean_text("CLN".to_string()));
    }

    #[test]
    fn test_clean_string() {
        assert_eq!("ADN", get_clean_text(r#"<span id="ID_10161034" class="SD">ADN</span><span class="sdParams" style="display: none;">TVOR;CODE_ID;111</span>"#.to_string()));
    }

    #[test]
    fn test_clean_multiline_string() {
        assert_eq!("ADN\nL2", get_clean_text(r#"<span id="ID_10161034" class="SD">ADN</span><span class="sdParams" style="display: none;">TVOR;CODE_ID;111</span><br /><span id="ID_10161034" class="SD">L2</span><span class="sdParams" style="display: none;">TVOR;CODE_ID;111</span>"#.to_string()));
    }

    #[test]
    fn test_complex_string() {
        let str = r#"
        <strong>
        <span class="SD" id="ID_10161012">ABERDEEN</span><span class="sdParams" style="display: none;">TVOR;TXT_NAME;111</span>
      </strong>
    <p class="line">
      <span class="SD" id="ID_10161015">VOR</span><span class="sdParams" style="display: none;">TVOR;CODE_TYPE;111</span>/DME<br/><span class="SD" id="ID_10161019">0.95°W</span><span class="sdParams" style="display: none;">TVOR;VAL_MAG_VAR;111</span> (<span class="SD" id="ID_10161022">2022</span><span class="sdParams" style="display: none;">TVOR;DATE_MAG_VAR;111</span>)<br/><span class="SD" id="ID_10161026">2.00°W</span><span class="sdParams" style="display: none;">TVOR;VAL_DECLINATION;111</span> (<span class="SD" id="ID_10161029">2018</span><span class="sdParams" style="display: none;">TVOR;CUSTOM_ATT1;111</span>)</p></td><td id="ID_10161031">
        "#;

        let intended_result = "ABERDEEN\nVOR/DME\n0.95°W(2022)\n2.00°W(2018)";

        assert_eq!(intended_result, get_clean_text(str.to_string()));
    }

    #[test]
    fn test_parse_frequency() {
        assert_eq!(123, parse_frequency("123 kHz").unwrap());
        assert_eq!(123000, parse_frequency("123 MHz").unwrap());
        assert_eq!(123456, parse_frequency("123.456 MHz").unwrap());
        assert_eq!(123456, parse_frequency("123456 kHz").unwrap());
        assert_eq!(123000, parse_frequency("123MHz").unwrap());
    }

    #[test]
    fn test_parse_elevation() {
        assert_eq!(10, parse_elevation("10ft").unwrap());
        assert_eq!(10, parse_elevation("10 ft").unwrap());
        assert_eq!(10, parse_elevation("10 FT").unwrap());
        assert!(parse_elevation("10M").is_err());
    }

    #[test]
    fn test_parse_latlong() {
        assert_eq!(
            (Some(57.1209), Some(2.1153)),
            parse_latlong("571209N 0021153E").unwrap()
        );
        assert_eq!(
            (Some(57.1209), Some(-2.1153)),
            parse_latlong("571209N 0021153W").unwrap()
        );
        assert_eq!(
            (Some(-57.1209), Some(2.1153)),
            parse_latlong("571209S 0021153E").unwrap()
        );
        assert_eq!(
            (Some(-57.1209), Some(-2.1153)),
            parse_latlong("571209S 0021153W").unwrap()
        );

        assert_eq!(
            (Some(57.1209), Some(2.1153)),
            parse_latlong("571209n 0021153e").unwrap()
        );
        assert_eq!(
            (Some(57.1209), Some(-2.1153)),
            parse_latlong("571209n 0021153w").unwrap()
        );
        assert_eq!(
            (Some(-57.1209), Some(2.1153)),
            parse_latlong("571209s 0021153e").unwrap()
        );
        assert_eq!(
            (Some(-57.1209), Some(-2.1153)),
            parse_latlong("571209s 0021153w").unwrap()
        );

        assert_eq!((Some(57.1209), None), parse_latlong("571209N").unwrap());
        assert_eq!((Some(-57.1209), None), parse_latlong("571209S").unwrap());
        assert_eq!((None, Some(2.1153)), parse_latlong("0021153E").unwrap());
        assert_eq!((None, Some(-2.1153)), parse_latlong("0021153W").unwrap());

        assert_eq!(
            (Some(57.120962), None),
            parse_latlong("571209.62N").unwrap()
        );
        assert_eq!(
            (None, Some(2.115312)),
            parse_latlong("0021153.12E").unwrap()
        );
    }
}
