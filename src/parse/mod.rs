use std::collections::VecDeque;

use airac::AIRAC;
use async_trait::async_trait;
use ego_tree::iter::{Edge, Traverse};
use scraper::{Html, Node};

use crate::eaip::EAIP;

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
    /// The error this parser will produce when failed
    type Error;

    /// Fetch the data from the given eAIP for the given AIRAC.
    async fn from_eaip(eaip: &EAIP, airac: AIRAC) -> Result<Self::Output, Self::Error>;

    /// Fetch the data from the given eAIP for the current AIRAC.
    async fn from_current_eaip(eaip: &EAIP) -> Result<Self::Output, Self::Error> {
        Self::from_eaip(eaip, AIRAC::current()).await
    }
}

/// The trait for all eAIP data parsers
pub trait Parser<'a> {
    /// The type this parser will output when successful
    type Output;
    /// The error this parser will produce when failed
    type Error;

    /// Parse the given HTML data into the type given by `Self::Output`.
    fn parse(data: &'a str) -> Result<Self::Output, Self::Error>;
}

pub(crate) fn get_clean_text(html_frag: String) -> String {
    let frag = Html::parse_fragment(&html_frag);
    get_clean_text_traverse(frag.root_element().traverse())
        .trim()
        .to_string()
}

pub(crate) fn get_clean_text_traverse(traverse: Traverse<'_, Node>) -> String {
    let mut s = String::new();
    let mut ignore_chain = VecDeque::new();
    for edge in traverse {
        match edge {
            Edge::Open(node) => {
                match node.value() {
                    Node::Element(e) => {
                        // if <br>, newline
                        if e.name() == "br" || e.name() == "p" {
                            if !s.ends_with("\n") {
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
                        if ignore_chain.len() == 0 {
                            s += &*t.trim();
                        }
                    }
                    _ => (),
                }
            }
            Edge::Close(node) => {
                // if matches end of ignore chain, pop_front
                if let Node::Element(e) = node.value() {
                    if !s.ends_with("\n") {
                        let inline_elems = vec!["span", "strong", "i", "em"];
                        if !inline_elems.contains(&&*e.name().to_lowercase()) {
                            s += "\n";
                        }
                    }

                    if ignore_chain.len() > 0 {
                        if e.name() == ignore_chain[0] {
                            ignore_chain.pop_front().unwrap();
                        }
                    }
                }
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::get_clean_text;

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
}
