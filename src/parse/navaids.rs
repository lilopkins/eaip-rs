use table_extract::Table;

use crate::parse::{get_clean_text, parse_frequency};
use crate::prelude::*;
use async_trait::async_trait;

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
        let table = Table::find_first(data);
        if table.is_none() {
            return Err(Error::CannotScrapeData("base table is not present"));
        }
        let table = table.unwrap();

        let mut navaids = Vec::new();
        'row: for row in &table {
            let mut i = 0;

            let mut navaid = NavAid::default();
            for cell in row {
                if i == 0 {
                    // Column 1 contains name and type
                    let clean = get_clean_text(cell.clone());
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
                    navaid.id = get_clean_text(cell.clone());
                } else if i == 2 {
                    // Column 3 contains frequency
                    let clean = get_clean_text(cell.clone());
                    navaid.frequency_khz = parse_frequency(clean)?;
                } else if i == 4 {
                    // Column 5 contains lat long
                    let clean = get_clean_text(cell.clone());
                    let (lat, lon) = parse_latlong(clean)?;
                    if let Some(lat) = lat {
                        navaid.latitude = lat;
                    }
                    if let Some(lon) = lon {
                        navaid.longitude = lon;
                    }
                } else if i == 5 {
                    // Column 6 contains elevation
                    let clean = get_clean_text(cell.clone());
                    navaid.elevation = parse_elevation(clean).unwrap_or(0);
                }

                i += 1;
            }

            if navaid != NavAid::default() {
                navaids.push(navaid);
            }
        }

        Ok(navaids)
    }
}
