use crate::parse::{get_clean_text, parse_frequency};
use crate::prelude::*;
use async_trait::async_trait;
use scraper::{Html, Selector};

use super::{parse_elevation, parse_latlong};

/// A list of radio-based navaids that can be parsed with a [`Parser`] from data from
/// an [`EAIP`](crate::eaip::EAIP).
pub type Navaids = Vec<NavAid>;

#[async_trait]
impl FromEAIP for Navaids {
    type Output = Self;

    async fn from_eaip(eaip: &EAIP, airac: airac::AIRAC) -> Result<Self::Output> {
        let page = Part::EnRoute(ENR::RadioNavAids(1));
        let data = eaip.get_page(airac, page, EAIPType::HTML).await?;
        let navaids = Navaids::parse(&data)?;
        Ok(navaids)
    }
}

impl<'a> Parser<'a> for Navaids {
    type Output = Self;

    fn parse(data: &'a str) -> Result<Self::Output> {
        let html = Html::parse_document(data);
        let table_content_selector = Selector::parse("table > tbody").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        let table = html
            .select(&table_content_selector)
            .next()
            .ok_or(Error::CannotScrapeData("base table not present"))?;

        let mut navaids = Vec::new();
        'row: for row in table.select(&tr_selector) {
            let mut navaid = NavAid::default();
            for (i, cell) in row.select(&td_selector).enumerate() {
                if i == 0 {
                    // Column 1 contains name and type
                    let clean = get_clean_text(cell.inner_html());
                    let lines = clean.split('\n').collect::<Vec<&str>>();
                    navaid.name = lines[0].trim().to_string();
                    let kind = lines[1].trim();
                    let typ = if kind == "VOR" {
                        NavAidKind::VOR
                    } else if kind == "VOR/DME" {
                        NavAidKind::VORDME
                    } else if kind == "DME" {
                        NavAidKind::DME
                    } else if kind == "NDB" {
                        NavAidKind::NDB
                    } else if kind.is_empty() {
                        continue 'row;
                    } else {
                        panic!("Unknown NavAidKind: {}", kind)
                    };
                    navaid.kind = typ;
                } else if i == 1 {
                    // Column 2 contains ID
                    navaid.id = get_clean_text(cell.inner_html());
                } else if i == 2 {
                    // Column 3 contains frequency
                    let clean = get_clean_text(cell.inner_html());
                    navaid.frequency_khz = parse_frequency(clean)?;
                } else if i == 4 {
                    // Column 5 contains lat long
                    let clean = get_clean_text(cell.inner_html());
                    let (lat, lon) = parse_latlong(clean)?;
                    if let Some(lat) = lat {
                        navaid.latitude = lat;
                    }
                    if let Some(lon) = lon {
                        navaid.longitude = lon;
                    }
                } else if i == 5 {
                    // Column 6 contains elevation
                    let clean = get_clean_text(cell.inner_html());
                    navaid.elevation = parse_elevation(clean).unwrap_or(0);
                }
            }

            if navaid != NavAid::default() {
                navaids.push(navaid);
            }
        }

        Ok(navaids)
    }
}
